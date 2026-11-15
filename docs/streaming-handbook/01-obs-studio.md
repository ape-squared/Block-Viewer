# 01 — OBS Studio (complete beginner research)

**Official sources:** [OBS Quickstart](https://obsproject.com/kb/quickstart), [Sources guide](https://obsproject.com/kb/sources-guide), [Audio Mixer](https://obsproject.com/kb/audio-mixer-guide), [Filters](https://obsproject.com/kb/filters-guide), [Streaming settings](https://obsproject.com/kb/streaming-settings), [Recording](https://obsproject.com/kb/recording-settings), [Auto-Configuration Wizard](https://obsproject.com/kb/auto-configuration-wizard), [Troubleshooting](https://obsproject.com/kb/troubleshooting).

**Label legend:** Verified = from OBS docs. Practical = class recommendation.

---

## 1. What OBS is (plain English)

**OBS Studio** (Open Broadcaster Software) is a free app that:

1. Collects video and audio from your PC (game/viewer window, desktop, webcam, mic).
2. Composites them into scenes (layouts).
3. Encodes that into a stream (to Kick, Twitch, YouTube, etc.) and/or a local recording file.

It does **not** host your stream. Platforms like Kick receive the encoded video over RTMP/RTMPS using a **stream key**.

**Why beginners start here:** One tool for capture, layout, audio, recording, and streaming. Industry-standard; huge help ecosystem.

---

## 2. Download & install

### Steps (Windows — typical class path)

1. Open https://obsproject.com/ and download **OBS Studio** for Windows.
2. Run the installer. Prefer the default install location unless IT policy says otherwise.
3. Launch OBS Studio.
4. On first launch, allow the **Auto-Configuration Wizard** (recommended).

**macOS / Linux:** Same site; pick the matching build. Capture types differ slightly (e.g. macOS Screen Capture permissions).

**Common mistake:** Downloading random “OBS packs” from unofficial sites. Stick to obsproject.com.

**Why it matters:** Unofficial builds may bundle malware or broken plugins.

---

## 3. First-time Auto-Configuration Wizard

**Verified concept:** Wizard tests your hardware and suggests canvas/output resolution, FPS, and bitrate for streaming or recording.  
Source: https://obsproject.com/kb/auto-configuration-wizard

### What to choose (practical)

| Question | Beginner pick | Why |
| --- | --- | --- |
| Optimize for | **Streaming** (if going live soon) or Recording | Sets bitrate targets appropriately |
| Resolution preference | Prefer **prefer high quality** if upload ≥ 6 Mbps; otherwise balanced | Upload bandwidth is the bottleneck for live |
| FPS | **30** for beginners; 60 only if CPU/GPU + upload are comfortable | Lower FPS = smoother encode on weak PCs |
| Service | Prefer **custom** later when adding Kick; wizard may list common services | Kick often uses custom RTMPS — see `02-kick.md` |

After wizard: note Settings → Video (Base/Canvas vs Output) and Settings → Output.

---

## 4. Scenes and sources

### Terminology

- **Scene** — A full layout you can switch to (e.g. “Starting Soon”, “Live”, “BRB”, “Ending”).
- **Source** — One layer inside a scene (game capture, image, webcam, text, audio).

**Verified:** Sources guide — https://obsproject.com/kb/sources-guide

### Mental model

Think of a scene as a stage. Sources are actors and props stacked bottom → top. Order in the sources list = layering (top of list draws on top).

### Practical scene set for class

Create four scenes before going live:

1. **Starting Soon** — full-bleed image/video + soft music (optional) + optional countdown text.
2. **Live** — Block Viewer / game capture + webcam + mic + optional overlay.
3. **BRB** — simple full-bleed “be right back”.
4. **Ending** — thanks / socials / Discord.

**Why:** Instant switches look professional; you never leave viewers on a frozen desktop while you fix something.

---

## 5. Adding capture sources

### 5a. Game Capture

**What:** Hooks into a fullscreen/game process for efficient capture.  
**Docs:** https://obsproject.com/kb/game-capture-source

**Steps (generic):**

1. Scene = Live → + → **Game Capture**.
2. Mode: Capture specific window / Capture any fullscreen application (per OBS version wording).
3. Select the target executable/window when prompted.
4. OK → resize/position.

**Block Viewer note [G1/G5]:** Block Viewer is a **Vulkan** client. OBS Game Capture injects hooks; BV bug-reporting docs list **OBS game capture** among overlay hooks that can affect sessions (`docs/BUG_REPORTING.md`).  

**Practical class guidance:**

- Without BV **Streamer mode**: try **Window Capture** first, then Display/Game Capture.  
- With **Streamer mode** on (client goes fullscreen — PR #82): expect Window Capture to struggle; test **Display Capture** of that monitor and/or **Game Capture**.  
- If Game Capture is stable for the student, they may keep it — do not invent a hard ban. Mark as **test combinations**.

### 5b. Display Capture

**What:** Captures an entire monitor.  
**Docs:** https://obsproject.com/kb/display-capture-source

**Use when:** Window/Game capture fails; or you need to show multiple windows.

**Risk:** Shows passwords, Discord DMs, stream keys — teach “clean desktop” habit.

### 5c. Window Capture

**What:** Captures one window (e.g. Block Viewer).  
**Docs:** https://obsproject.com/kb/window-capture-source

**Practical for Block Viewer:** Often the safest beginner path. If the window goes black when minimized, keep BV visible/on the captured display.

### 5d. Webcam (Video Capture Device)

1. + → **Video Capture Device**.
2. Select camera.
3. Set resolution (1080p or 720p — match what the cam can do).
4. Crop/position as a corner picture-in-picture on Live scene.

**Permissions:** Windows Privacy → Camera; macOS Camera permission for OBS.

### 5e. Microphone & desktop audio

OBS usually creates:

- **Mic/Aux** — your microphone
- **Desktop Audio** — system/game/viewer sound

**Settings → Audio:** Confirm correct devices.

**Windows tip:** If Desktop Audio is silent, check Windows sound settings / exclusive mode / wrong output device.

### 5f. Other useful sources

| Source | Use |
| --- | --- |
| Image | Overlay PNGs, Starting Soon art |
| Image Slide Show | Rotating BRB tips |
| Media Source | Intro/outro video, music bed (watch copyright) |
| Text (GDI+) | “Starting soon”, socials |
| Browser Source | Alerts, chat overlays (advanced) |
| Color Source | Solid background behind webcam |

---

## 6. Desktop audio & microphone setup

### Goals (practical)

- Mic peaks roughly **−12 dB to −6 dB** when speaking normally (yellow, not slamming red).
- Desktop/game audio a bit under the voice so chat can hear you.
- No constant red clipping.

### Steps

1. Speak → watch Mic meter in Audio Mixer.
2. Play in-world / desktop sound → watch Desktop meter.
3. Use gear icon → **Advanced Audio Properties** for sync offset if lipsync is off (advanced).

**Why levels matter:** Too quiet = viewers turn you down in their head and leave. Too loud = distortion and platform loudness issues.

---

## 7. Audio Mixer

**Docs:** https://obsproject.com/kb/audio-mixer-guide

Controls per source:

- Volume fader
- Mute
- Gear → filters, properties

**Monitoring** (headphones): gear → Advanced Audio Properties → Audio Monitoring:

| Mode | Meaning |
| --- | --- |
| Monitor Off | You don’t hear that source in headphones via OBS |
| Monitor Only | Hear it; not sent to stream (careful) |
| Monitor and Output | Hear it and send to stream |

**Echo / double audio:** Often Monitor and Output + hearing desktop through speakers while mic picks it up. Fix: headphones; or Monitor Off for Desktop; lower speaker volume.

---

## 8. Filters (mic chain for beginners)

**Docs:** https://obsproject.com/kb/filters-guide

**Practical order on Mic** (right-click Mic → Filters):

1. **Noise Suppression** (RNNoise if available) — cuts fan/AC hiss.  
   Why: Cleaner voice without buying a booth.
2. **Noise Gate** (optional) — mutes below threshold when you’re silent.  
   Why: Hides keyboard between sentences; set carefully so soft speech isn’t cut.
3. **Compressor** — evens loud/soft speech.  
   Why: Laughs don’t explode; whispers stay audible.
4. **Limiter** — safety ceiling so peaks don’t clip.  
   Why: Last defense against distortion.
5. **Gain** — only if the whole chain is still too quiet after correct Windows mic level.

**Common mistake:** Stacking five noise suppressors + huge gain → robotic voice. Start light.

**Do not** put random “pro preset” packs from untrusted sites on day one.

---

## 9. Video, FPS, bitrate, encoder (streaming settings)

**Docs:** https://obsproject.com/kb/streaming-settings

### Key concepts

| Setting | Plain English | Beginner starting point (practical) |
| --- | --- | --- |
| Base (Canvas) Resolution | Working layout size | 1920×1080 |
| Output (Scaled) Resolution | What you send | 1920×1080 or 1280×720 if upload is weak |
| FPS | Frames per second | 30 |
| Bitrate | Data per second to the platform | Start ~2500–4500 kbps for 720p30; ~4500–6000 for 1080p30 if upload allows |
| Encoder | Who compresses video | Prefer **hardware** (NVENC / AMF / QuickSync) if available; else x264 |
| Rate control | How bitrate is held | CBR for most live platforms |
| Keyframe interval | Reference frame spacing | Often **2 seconds** (platform-dependent — Kick docs may specify) |

**Why bitrate matters:** Higher = sharper but needs stable upload. Exceeding upload → dropped frames / rubber-banding for viewers.

**Test:** Speed test; leave ~20–40% headroom under upload for stream bitrate.

### Connecting to Kick

Usually Settings → Stream → Service **Custom** → server URL + stream key from Kick dashboard. Details in `02-kick.md`. **Never commit keys to git or Discord.**

---

## 10. Recording locally

**Docs:** https://obsproject.com/kb/recording-settings

### Practical beginner setup

1. Settings → Output → Recording.
2. Recording path: a folder with free disk space (SSD preferred).
3. Format: **mkv** (safer if OBS crashes; remux to mp4 via OBS File → Remux) or mp4 if student understands crash risk.
4. Encoder: can match stream or use higher quality for local VODs.
5. Optional: **Replay Buffer** — hotkey to save last N seconds (great for clips).

**Why local record:** Platform VODs get deleted; clips for TikTok/Reels need clean sources; troubleshooting (“was that my encode?”).

---

## 11. Scene recipes (class templates)

### Starting Soon

- Image or Media Source (looping, muted or licensed music).
- Text: “Starting soon” + stream title.
- Optional: Browser source for low-key chat (advanced).

### Live (Block Viewer)

- Window Capture (Block Viewer) or tested Game Capture.
- Video Capture Device (webcam) — corner, not covering critical UI.
- Mic + Desktop Audio.
- Optional thin overlay / webcam frame PNG (see `06-graphics-branding.md`).
- Optional Browser alert source.

### BRB

- Full-bleed image; short text; optional soft music.
- Mic muted or very low if AFK.

### Ending

- Thanks image; Discord / socials; “follow for next stream”.
- Stop streaming only after a few seconds on this scene (practical courtesy).

---

## 12. Troubleshooting (OBS-focused)

See also `08-troubleshooting.md`.

| Symptom | Likely causes | Fixes |
| --- | --- | --- |
| Black screen (Game/Window) | Wrong capture type, exclusive fullscreen, GPU multi-adapter, Vulkan hook | Try Window Capture; borderless window; run OBS/game on same GPU; update GPU drivers; see OBS troubleshooting KB |
| No mic | Wrong device, muted, Windows privacy, exclusive mode | Recheck Audio devices; unmute; Privacy → Mic |
| No desktop audio | Wrong output device, app using different endpoint | Match Windows default playback; restart OBS after device change |
| Dropped frames (network) | Bitrate > upload, Wi‑Fi | Lower bitrate; wired Ethernet; close uploads |
| Encoding overloaded | Encoder too heavy | Lower res/FPS; switch to NVENC; close Chrome tabs |
| Choppy preview only | Preview scale — check actual stream | Confirm with test stream / local record |

---

## 13. Beginner recommended “day one” config (practical)

```
Canvas: 1920×1080
Output: 1280×720 (upgrade to 1080 when stable)
FPS: 30
Encoder: NVENC / AMF / QSV if available, else x264 veryfast
Bitrate: 3500 kbps CBR (adjust to upload)
Keyframe: 2s
Mic filters: Noise Suppression → Compressor → Limiter
Scenes: Starting Soon, Live, BRB, Ending
Capture: Window Capture for Block Viewer first; re-test with Streamer mode (Display/Game if needed)
Record: MKV + Replay Buffer 60–120s
```

**Why 720p30 first:** Fewer moving parts; successful first stream beats a glamorous failed 1080p60.

**Block Viewer Streamer mode:** See `04-block-viewer.md` §E0 — cleans immersive UI; does not replace OBS.

---

## 14. Screenshot needs

See `00-meta-sources-gaps.md` items 1–8.

---

## 15. What Claude should write (chapter goals)

- Zero assumed jargon; define scene/source/bitrate once.
- Numbered installs and “add source” walks.
- Explicit “why” under each recommended setting.
- Call out Block Viewer capture uncertainty honestly.
- End with a 10-minute “dry run” checklist (record locally, don’t go live yet).
