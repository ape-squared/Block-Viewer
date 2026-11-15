# 08 — Troubleshooting & optimization (research matrix)

Format for each issue: **Symptoms → Likely causes → Step-by-step fixes → Prevention.**  
Cross-reference OBS KB: https://obsproject.com/kb/troubleshooting

---

## 1. OBS not capturing gameplay / Block Viewer (black or frozen)

**Symptoms:** Source shows black; last frame stuck; works in preview then dies.

**Likely causes:**

- Wrong capture type for Vulkan/DirectX app  
- Exclusive fullscreen / **Block Viewer Streamer mode** (forces fullscreen)  
- Multi-GPU (laptop): OBS on iGPU, game on dGPU  
- Window minimized / hardware overlay  
- OBS Game Capture hook conflict  

**Fixes:**

1. If Streamer mode is **off**: try **Window Capture** → select Block Viewer; keep windowed/borderless and visible.  
2. If Streamer mode is **on** (or other fullscreen): try **Display Capture** of that monitor, then **Game Capture**.  
3. Windows Graphics Settings: set OBS **and** Block Viewer to the same high-performance GPU.  
4. Update GPU drivers; restart PC.  
5. If using Game Capture: temporarily disable Discord/Steam/RTSS overlays.  
6. Capture a local recording to confirm it’s not just a preview glitch.

**Prevention:** Standardize on a known-good capture method per PC (with and without Streamer mode); document it in the student’s one-pager. See BV overlay hooks note in `docs/BUG_REPORTING.md` and Streamer mode notes in `04-block-viewer.md`.

---

## 2. No microphone audio

**Symptoms:** Mic meter flat; viewers hear only game.

**Likely causes:** Muted; wrong device; OS privacy; exclusive mode; cable/USB issue; Filters gate stuck closed.

**Fixes:**

1. Unmute Mic in OBS mixer.  
2. Settings → Audio → Mic/Aux device = correct input.  
3. Windows Privacy → Microphone → allow desktop apps / OBS.  
4. Speak in Windows sound settings level meter first.  
5. Disable exclusive mode on the device (Advanced).  
6. Bypass filters temporarily (Noise Gate threshold too high is common).  
7. Try another USB port / headset.

**Prevention:** Pre-stream “count to five” meter check every time.

---

## 3. Desktop audio not working

**Symptoms:** World/music silent on stream; mic OK.

**Likely causes:** Wrong desktop device; app output routed elsewhere (Voicemeeter mess); Windows update reset defaults.

**Fixes:**

1. Settings → Audio → Desktop Audio = correct playback device.  
2. Click speaker icon in Windows → confirm default device.  
3. In BV/game, ensure not muted.  
4. Restart OBS after device changes.  
5. Advanced Audio Properties: ensure Desktop isn’t set to Monitor Only by mistake when you expected Output.

**Prevention:** Avoid exotic virtual audio cables until single-device setup works.

---

## 4. Echo and double audio

**Symptoms:** Hollow echo; hearing yourself twice; chat complains.

**Likely causes:**

- Speakers open + mic monitoring  
- Desktop audio Monitor and Output while headphones also play  
- Two mics active  
- Kick VOD + local speakers feedback (rare)

**Fixes:**

1. Wear headphones.  
2. Mic Monitoring = Monitor Off (or Only if you know why).  
3. Mute second mic devices.  
4. Lower Desktop volume; don’t point mic at speakers.

**Prevention:** Headset-first policy for class.

---

## 5. Stream lag (viewers) / local feel fine

**Symptoms:** Viewers rubber-band; you’re smooth locally.

**Likely causes:** Bitrate > upload; Wi‑Fi; ISP bufferbloat; platform ingest congestion; too many Multistream destinations.

**Fixes:**

1. OBS Stats → note **network** dropped frames.  
2. Lower bitrate 20–30%; try 720p30.  
3. Switch to Ethernet.  
4. Pause cloud backups/uploads during stream.  
5. Disable extra Multistream destinations.  
6. Test different Kick ingest if dashboard offers choices **[NEEDS CONFIRMATION]**.

**Prevention:** Speed test; set bitrate with headroom; wired connection.

---

## 6. Dropped frames

**Symptoms:** OBS Stats show dropped frames (network or rendering/encoding).

**Split diagnosis:**

| Type | Meaning | Direction |
| --- | --- | --- |
| Network | Upload can’t carry bitrate | Lower bitrate / wired |
| Rendering | GPU busy drawing scenes | Simplify; lower BV settings |
| Encoding | Encoder overloaded | NVENC; lower res/FPS; preset faster |

**Prevention:** Stable 720p30 profile before chasing 1080p60.

---

## 7. Encoding overload

**Symptoms:** Laggy stream; “encoding overloaded” message; high CPU.

**Fixes:**

1. Switch x264 → **NVENC/AMF/QSV**.  
2. Output 1280×720, 30 FPS.  
3. Close browser tabs / Discord hardware accel experiments.  
4. Lower BV graphics / draw distance.  
5. Disable unnecessary Browser Sources.

**Prevention:** Hardware encoder as default for class laptops with discrete GPUs.

---

## 8. Internet connection issues

**Symptoms:** Disconnects; “failed to connect”; unstable meters.

**Fixes:**

1. Ethernet; disable VPN unless required.  
2. Router QoS / reboot modem.  
3. Hotspot test to isolate home ISP.  
4. Corporate firewall may block RTMPS — try alternate network.  
5. Confirm Kick/platform status.

**Prevention:** Pre-stream 5-minute connection test; know a backup network.

---

## 9. Kick connection problems

See also `02-kick.md`.

**Symptoms:** OBS says streaming; Kick not live — or immediate fail.

**Fixes:**

1. Re-copy **server + key**; regenerate key if unsure.  
2. Service = Custom (or current Kick-recommended).  
3. Clock sync (wrong Windows time can break TLS oddly — rare).  
4. Update OBS.  
5. Temporarily disable firewall for test.  
6. Confirm correct Kick account in browser.

**Prevention:** Key only in OBS; written go-live checklist.

---

## 10. Multistream problems

See `03-aitum-multistream.md`.

**Symptoms:** One site live, another not; all fail only when Multistream enabled.

**Fixes:**

1. Validate each destination alone.  
2. Re-OAuth / refresh keys.  
3. Match Aitum build to OBS version.  
4. Reduce destinations; watch upload.  
5. Fall back to Kick-only for the show.

**Prevention:** Don’t debut Multistream on a community event night.

---

## 11. Recording problems

**Symptoms:** No file; corrupt mp4; huge disk use; audio desync.

**Fixes:**

1. Check recording path permissions / disk space.  
2. Prefer **MKV** then remux.  
3. Same encoder settings sanity as stream.  
4. Sync offset in Advanced Audio if consistently late.  
5. Disable Replay Buffer if disk can’t keep up; or shorten buffer.

**Prevention:** Dedicated recordings drive/folder; clear old VODs.

---

## 12. Block Viewer–specific problems

Sources: `/docs/reporting-bugs/`, Poser/LiveMesh docs as relevant; **no official OBS page**.

| Issue | Direction |
| --- | --- |
| Crash when OBS starts capturing | Disable overlays; change capture type; report with overlay hooks listed |
| Black frame only after Streamer mode | Fullscreen vs Window Capture — use Display/Game Capture; see §1 |
| Streamer mode unavailable | Build older than PR #82 — update from official download |
| Login / grid issues | Outside streaming — SL account/grid status; update BV |
| Poser can’t save / confusing UI | Follow https://blockviewer.org/docs/poser/ exactly; bring screenshots to class |
| LiveMesh broken | https://blockviewer.org/docs/livemesh/ + Blender version notes in that doc |
| Performance only while streaming | Encode+render contention — lower settings / NVENC |
| Student expects in-BV “Go Live” | Educate: Streamer mode cleans UI; OBS + Kick broadcast |

**Bug filing:** https://blockviewer.org/docs/reporting-bugs/

---

## 13. Optimization quick wins (cheat sheet)

1. 720p30 + hardware encode + bitrate with headroom.  
2. Window Capture BV; overlays off.  
3. Headphones; clean mic chain.  
4. Wired internet.  
5. Four scenes; no alert spam.  
6. One platform until stable.  
7. Local 1-minute test record before every first-of-day live.

---

## 14. Collaborative troubleshooting (class)

Students should bring to sessions:

- OBS Stats screenshot (dropped frames type visible)  
- Capture method used  
- Overlay list (Discord/Steam/RTSS/Game Capture)  
- Upload speed result  
- Kick error text (no stream keys)  
- BV build/version from Updates/download  

See `09-support-structure.md`.
