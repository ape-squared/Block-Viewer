//! In-viewer camera recording: window-size scene frames encoded to an MP4
//! (H.264) next to the stills folder. Windows uses Media Foundation; other
//! platforms refuse the start with a clear error. The viewfinder drives
//! start / stop; the app asks `capture_scene` on a cadence and feeds RGBA.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::time::{Duration, Instant};

/// Target frames written per second (capture requests follow this).
pub const TARGET_FPS: u32 = 24;
/// Long edge cap before encode (keeps bitrate and GPU readback sane).
pub const MAX_EDGE: u32 = 1280;
/// Hard stop so a forgotten REC cannot fill the disk.
pub const MAX_SECONDS: u32 = 10 * 60;

/// What the app should do for recording this frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tick {
    Idle,
    /// Ask `renderer.capture_scene` (world only — no viewfinder chrome).
    RequestCapture,
    Recording {
        elapsed: Duration,
        frames: u32,
    },
}

/// Session controller on the UI thread; encode work runs on a worker.
#[derive(Default)]
pub struct VideoRecorder {
    phase: Phase,
    /// Last finish result for the app to toast once.
    pub finished: Option<Result<PathBuf, String>>,
}

#[derive(Default)]
enum Phase {
    #[default]
    Idle,
    Live(Live),
    Finishing {
        done: Receiver<Result<PathBuf, String>>,
        started: Instant,
        frames: u32,
    },
}

struct Live {
    started: Instant,
    frames: u32,
    capture_pending: bool,
    next_due: Instant,
    tx: Sender<WorkerMsg>,
    done: Receiver<Result<PathBuf, String>>,
}

enum WorkerMsg {
    Frame { width: u32, height: u32, rgba: Vec<u8> },
    Stop,
}

impl VideoRecorder {
    pub fn recording(&self) -> bool {
        !matches!(self.phase, Phase::Idle)
    }

    pub fn elapsed(&self) -> Option<Duration> {
        match &self.phase {
            Phase::Live(s) => Some(s.started.elapsed()),
            Phase::Finishing { started, .. } => Some(started.elapsed()),
            Phase::Idle => None,
        }
    }

    /// Begin a session that writes under `folder` (created if needed).
    pub fn start(&mut self, folder: &Path) -> Result<(), String> {
        if !matches!(self.phase, Phase::Idle) {
            return Err("already recording".into());
        }
        #[cfg(not(windows))]
        {
            let _ = folder;
            return Err("in-viewer video recording is only available on Windows".into());
        }
        #[cfg(windows)]
        {
            std::fs::create_dir_all(folder).map_err(|e| format!("video folder: {e}"))?;
            let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
            let path = folder.join(format!("Block-{stamp}.mp4"));
            let (tx, rx) = mpsc::channel::<WorkerMsg>();
            let (done_tx, done_rx) = mpsc::channel();
            let path_worker = path.clone();
            std::thread::Builder::new()
                .name("video-record".into())
                .spawn(move || {
                    let result = run_worker(path_worker, rx);
                    let _ = done_tx.send(result);
                })
                .map_err(|e| format!("video worker: {e}"))?;
            let now = Instant::now();
            log::info!("camera video: recording to {}", path.display());
            self.phase = Phase::Live(Live {
                started: now,
                frames: 0,
                capture_pending: false,
                next_due: now,
                tx,
                done: done_rx,
            });
            Ok(())
        }
    }

    /// Ask the worker to finalize; the path (or error) lands in `finished`.
    pub fn stop(&mut self) {
        let Phase::Live(live) = std::mem::replace(&mut self.phase, Phase::Idle) else {
            return;
        };
        let _ = live.tx.send(WorkerMsg::Stop);
        self.phase = Phase::Finishing {
            done: live.done,
            started: live.started,
            frames: live.frames,
        };
        self.poll_finished();
    }

    pub fn poll_finished(&mut self) {
        let done = match &self.phase {
            Phase::Finishing { done, .. } => done,
            Phase::Live(live) => &live.done,
            Phase::Idle => return,
        };
        match done.try_recv() {
            Ok(result) => {
                self.phase = Phase::Idle;
                self.finished = Some(result);
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                self.phase = Phase::Idle;
                self.finished = Some(Err("video encoder stopped unexpectedly".into()));
            }
        }
    }

    /// Per-frame: request captures on cadence, surface status, collect finish.
    pub fn tick(&mut self) -> Tick {
        self.poll_finished();
        match &mut self.phase {
            Phase::Idle => Tick::Idle,
            Phase::Finishing { started, frames, .. } => Tick::Recording {
                elapsed: started.elapsed(),
                frames: *frames,
            },
            Phase::Live(live) => {
                if live.started.elapsed() >= Duration::from_secs(u64::from(MAX_SECONDS)) {
                    // Auto-stop at the cap (rebuild via stop).
                    let live = match std::mem::replace(&mut self.phase, Phase::Idle) {
                        Phase::Live(l) => l,
                        other => {
                            self.phase = other;
                            return Tick::Idle;
                        }
                    };
                    let _ = live.tx.send(WorkerMsg::Stop);
                    self.phase = Phase::Finishing {
                        done: live.done,
                        started: live.started,
                        frames: live.frames,
                    };
                    return self.tick();
                }
                let elapsed = live.started.elapsed();
                let frames = live.frames;
                if !live.capture_pending && Instant::now() >= live.next_due {
                    live.capture_pending = true;
                    return Tick::RequestCapture;
                }
                Tick::Recording { elapsed, frames }
            }
        }
    }

    /// A scene capture arrived; enqueue for encode.
    pub fn push_frame(&mut self, width: u32, height: u32, rgba: Vec<u8>) {
        let Phase::Live(live) = &mut self.phase else {
            return;
        };
        live.capture_pending = false;
        let interval = Duration::from_secs_f64(1.0 / f64::from(TARGET_FPS));
        live.next_due = Instant::now() + interval;
        if width < 2 || height < 2 || rgba.len() < (width as usize) * (height as usize) * 4 {
            return;
        }
        if live.tx.send(WorkerMsg::Frame { width, height, rgba }).is_ok() {
            live.frames += 1;
        }
    }

    /// Capture was requested but another path stole `captured`; retry soon.
    pub fn capture_missed(&mut self) {
        if let Phase::Live(live) = &mut self.phase {
            live.capture_pending = false;
            live.next_due = Instant::now();
        }
    }
}

#[cfg(windows)]
fn run_worker(path: PathBuf, rx: Receiver<WorkerMsg>) -> Result<PathBuf, String> {
    windows_encode::encode(path, rx)
}

#[cfg(windows)]
mod windows_encode {
    use super::{MAX_EDGE, TARGET_FPS, WorkerMsg};
    use std::os::windows::ffi::OsStrExt;
    use std::path::PathBuf;
    use std::sync::mpsc::Receiver;
    use windows::Win32::Media::MediaFoundation::{
        IMFMediaType, IMFSample, IMFSinkWriter, MF_MT_AVG_BITRATE, MF_MT_DEFAULT_STRIDE, MF_MT_FRAME_RATE, MF_MT_FRAME_SIZE,
        MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE, MF_MT_PIXEL_ASPECT_RATIO, MF_MT_SUBTYPE, MF_VERSION, MFCreateMediaType,
        MFCreateMemoryBuffer, MFCreateSample, MFCreateSinkWriterFromURL, MFMediaType_Video, MFSTARTUP_FULL, MFShutdown, MFStartup,
        MFVideoFormat_H264, MFVideoFormat_RGB32, MFVideoInterlace_Progressive,
    };
    use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize};
    use windows::core::PCWSTR;

    struct ComGuard(bool);
    impl Drop for ComGuard {
        fn drop(&mut self) {
            if self.0 {
                // SAFETY: paired with successful CoInitializeEx on this thread.
                unsafe { CoUninitialize() };
            }
        }
    }

    struct MfGuard;
    impl Drop for MfGuard {
        fn drop(&mut self) {
            // SAFETY: paired with MFStartup below.
            unsafe {
                let _ = MFShutdown();
            }
        }
    }

    pub(super) fn encode(path: PathBuf, rx: Receiver<WorkerMsg>) -> Result<PathBuf, String> {
        // SAFETY: COM + MF on this dedicated worker thread only.
        unsafe {
            let com_ok = CoInitializeEx(None, COINIT_MULTITHREADED).is_ok();
            let _com = ComGuard(com_ok);
            MFStartup(MF_VERSION, MFSTARTUP_FULL).map_err(|e| format!("MFStartup: {e}"))?;
            let _mf = MfGuard;

            let mut writer: Option<Writer> = None;
            let mut frames = 0u64;
            while let Ok(msg) = rx.recv() {
                match msg {
                    WorkerMsg::Frame { width, height, rgba } => {
                        let (w, h, pixels) = fit_frame(width, height, &rgba);
                        if writer.is_none() {
                            writer = Some(Writer::open(&path, w, h)?);
                        }
                        let wtr = writer.as_mut().expect("opened");
                        if (w, h) != (wtr.width, wtr.height) {
                            continue;
                        }
                        wtr.write_bgra(&pixels, frames)?;
                        frames += 1;
                    }
                    WorkerMsg::Stop => break,
                }
            }
            if let Some(wtr) = writer.take() {
                wtr.finish()?;
                if frames == 0 {
                    let _ = std::fs::remove_file(&path);
                    return Err("no frames were recorded".into());
                }
                Ok(path)
            } else {
                Err("recording stopped before the first frame".into())
            }
        }
    }

    struct Writer {
        sink: IMFSinkWriter,
        stream: u32,
        width: u32,
        height: u32,
        frame_duration: i64,
    }

    impl Writer {
        fn open(path: &std::path::Path, width: u32, height: u32) -> Result<Self, String> {
            let wide: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
            // SAFETY: COM/MF already started on this worker; `wide` is NUL-terminated
            // and lives for the call; media types are released with the writer.
            unsafe {
                let sink = MFCreateSinkWriterFromURL(PCWSTR(wide.as_ptr()), None, None).map_err(|e| format!("SinkWriter: {e}"))?;

                let out_type = MFCreateMediaType().map_err(|e| format!("media type: {e}"))?;
                set_video_common(&out_type, width, height)?;
                out_type
                    .SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_H264)
                    .map_err(|e| format!("H264: {e}"))?;
                out_type
                    .SetUINT32(&MF_MT_AVG_BITRATE, 6_000_000)
                    .map_err(|e| format!("bitrate: {e}"))?;
                let stream = sink.AddStream(&out_type).map_err(|e| format!("AddStream: {e}"))?;

                let in_type = MFCreateMediaType().map_err(|e| format!("input type: {e}"))?;
                set_video_common(&in_type, width, height)?;
                in_type
                    .SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32)
                    .map_err(|e| format!("RGB32: {e}"))?;
                in_type
                    .SetUINT32(&MF_MT_DEFAULT_STRIDE, width * 4)
                    .map_err(|e| format!("stride: {e}"))?;
                sink.SetInputMediaType(stream, &in_type, None)
                    .map_err(|e| format!("SetInputMediaType: {e}"))?;
                sink.BeginWriting().map_err(|e| format!("BeginWriting: {e}"))?;

                Ok(Self {
                    sink,
                    stream,
                    width,
                    height,
                    frame_duration: 10_000_000 / i64::from(TARGET_FPS),
                })
            }
        }

        /// `bgra` is already BGRA8 top-down at `width`×`height`.
        fn write_bgra(&self, bgra: &[u8], index: u64) -> Result<(), String> {
            let bytes = (self.width * self.height * 4) as usize;
            if bgra.len() < bytes {
                return Err("frame buffer too small".into());
            }
            // SAFETY: buffer is locked for `bytes`; `bgra` is valid for that length;
            // Unlock before the sample is written.
            unsafe {
                let buffer = MFCreateMemoryBuffer(bytes as u32).map_err(|e| format!("buffer: {e}"))?;
                let mut raw = std::ptr::null_mut();
                let mut max_len = 0u32;
                buffer.Lock(&mut raw, Some(&mut max_len), None).map_err(|e| format!("Lock: {e}"))?;
                if raw.is_null() || (max_len as usize) < bytes {
                    let _ = buffer.Unlock();
                    return Err("MF buffer lock failed".into());
                }
                std::ptr::copy_nonoverlapping(bgra.as_ptr(), raw, bytes);
                buffer.Unlock().map_err(|e| format!("Unlock: {e}"))?;
                buffer.SetCurrentLength(bytes as u32).map_err(|e| format!("length: {e}"))?;

                let sample: IMFSample = MFCreateSample().map_err(|e| format!("sample: {e}"))?;
                sample.AddBuffer(&buffer).map_err(|e| format!("AddBuffer: {e}"))?;
                let time = index as i64 * self.frame_duration;
                sample.SetSampleTime(time).map_err(|e| format!("time: {e}"))?;
                sample
                    .SetSampleDuration(self.frame_duration)
                    .map_err(|e| format!("duration: {e}"))?;
                self.sink
                    .WriteSample(self.stream, &sample)
                    .map_err(|e| format!("WriteSample: {e}"))?;
                Ok(())
            }
        }

        fn finish(self) -> Result<(), String> {
            // SAFETY: sink was BeginWriting'd; Finalize must run once on this thread.
            unsafe { self.sink.Finalize().map_err(|e| format!("Finalize: {e}")) }
        }
    }

    fn set_video_common(ty: &IMFMediaType, width: u32, height: u32) -> Result<(), String> {
        // SAFETY: `ty` is a live IMFMediaType from MFCreateMediaType on this thread.
        unsafe {
            ty.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)
                .map_err(|e| format!("major: {e}"))?;
            let size = ((width as u64) << 32) | u64::from(height);
            ty.SetUINT64(&MF_MT_FRAME_SIZE, size).map_err(|e| format!("frame size: {e}"))?;
            let rate = ((u64::from(TARGET_FPS)) << 32) | 1;
            ty.SetUINT64(&MF_MT_FRAME_RATE, rate).map_err(|e| format!("frame rate: {e}"))?;
            ty.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, (1u64 << 32) | 1)
                .map_err(|e| format!("PAR: {e}"))?;
            ty.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)
                .map_err(|e| format!("interlace: {e}"))?;
            Ok(())
        }
    }

    /// Downscale to `MAX_EDGE` and convert RGBA → BGRA; even dimensions for H.264.
    fn fit_frame(width: u32, height: u32, rgba: &[u8]) -> (u32, u32, Vec<u8>) {
        let long = width.max(height).max(1);
        let scale = if long > MAX_EDGE { MAX_EDGE as f32 / long as f32 } else { 1.0 };
        let w = ((width as f32 * scale).round() as u32).max(2) & !1;
        let h = ((height as f32 * scale).round() as u32).max(2) & !1;
        let mut out = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            let sy = ((y as f32 + 0.5) * (height as f32 / h as f32) - 0.5)
                .round()
                .clamp(0.0, (height - 1) as f32) as u32;
            for x in 0..w {
                let sx = ((x as f32 + 0.5) * (width as f32 / w as f32) - 0.5)
                    .round()
                    .clamp(0.0, (width - 1) as f32) as u32;
                let si = ((sy * width + sx) * 4) as usize;
                let di = ((y * w + x) * 4) as usize;
                out[di] = rgba[si + 2];
                out[di + 1] = rgba[si + 1];
                out[di + 2] = rgba[si];
                out[di + 3] = 0xFF;
            }
        }
        (w, h, out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_idle_without_session() {
        let mut r = VideoRecorder::default();
        assert_eq!(r.tick(), Tick::Idle);
        assert!(!r.recording());
    }

    #[test]
    fn push_without_session_is_ignored() {
        let mut r = VideoRecorder::default();
        r.push_frame(4, 4, vec![0; 64]);
        assert!(!r.recording());
    }
}
