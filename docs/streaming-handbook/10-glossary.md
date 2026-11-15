# 10 — Glossary of streaming terminology

Plain-English definitions for handbook readers. Keep this list alphabetized in the final book.

| Term | Meaning |
| --- | --- |
| **Alert** | On-stream graphic/sound when an event happens (follow, sub, donation). |
| **Aitum Multistream** | Tooling to send one production to multiple platforms. See `03`. |
| **Bitrate** | How much video/audio data you send per second (kbps). Higher = sharper + needs more upload. |
| **Block Viewer** | Open-source Second Life client with Vulkan renderer; [blockviewer.org](https://blockviewer.org/). |
| **BRB scene** | OBS layout shown when you’re temporarily away. |
| **Browser Source** | OBS source that renders a web page (alerts, widgets). |
| **Canvas (Base) resolution** | The resolution of your OBS layout workspace. |
| **CBR** | Constant bitrate — steady data rate; common for live streaming. |
| **Clip** | Short saved segment of a stream for social posts. |
| **Compressor (filter)** | Evens out loud and soft speech levels. |
| **Desktop Audio** | OBS input that captures system/game sound. |
| **Display Capture** | Captures an entire monitor. |
| **Dropped frames** | Frames not sent/rendered in time; causes stutter for viewers or local lag. |
| **Encoder** | Software/hardware that compresses video (x264, NVENC, AMF, QuickSync). |
| **Game Capture** | OBS source that hooks into a game/process for capture. |
| **Ingest** | Platform server URL that receives your stream. |
| **Keyframe interval** | How often a full reference frame is sent; often 2s for live. |
| **Kick** | Live streaming platform at kick.com. |
| **Limiter** | Filter that prevents audio peaks from exceeding a ceiling. |
| **LiveMesh** | Block Viewer ↔ Blender mesh workflow; `/docs/livemesh/`. |
| **Mic/Aux** | OBS microphone channel. |
| **Monitoring** | Hearing an audio source in your headphones via OBS. |
| **Noise gate** | Mutes audio below a threshold. |
| **Noise suppression** | Filter that reduces constant background noise. |
| **NVENC** | NVIDIA hardware video encoder. |
| **OBS Studio** | Free open-source app for recording and streaming. |
| **Output (Scaled) resolution** | Resolution actually sent to the platform or recording. |
| **Overlay** | Graphics drawn on top of your capture. |
| **Overlay hook** | Injected capture/overlay software (Discord, Steam, RTSS, OBS Game Capture) noted in BV bug reports. |
| **Poser** | Block Viewer local photo/pose workflow; `/docs/poser/`. |
| **Replay Buffer** | OBS feature saving the last N seconds on hotkey. |
| **RTMP / RTMPS** | Protocols used to send streams to platforms (S = TLS). |
| **Scene** | A full OBS layout you can switch to. |
| **Source** | A single layer inside a scene. |
| **Stream key** | Secret token authorizing your encoder to broadcast to your channel. **Never share.** |
| **Streamer mode (Block Viewer)** | Client layout for creators: Immersive UI + fullscreen; hides L$ balance, hover tips, name tags, and mouselook Esc hint. Does **not** broadcast by itself. See PR #82 / Preferences › Layout. |
| **VOD** | Video on demand — recorded stream playback after live. |
| **Vulkan** | Modern graphics API; Block Viewer uses a Vulkan renderer. |
| **Window Capture** | Captures a single window. |
| **x264** | CPU-based software H.264 encoder. |
