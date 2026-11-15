# 11 — FAQ bank (for handbook + classes)

Answers should stay short in the final book; point to chapters for steps.

---

## Getting started

**Q: I’ve never streamed. What do I install first?**  
A: OBS Studio → Block Viewer → Kick account. Multistream later. Chapters `01`, `04`, `02`.

**Q: Do I need a fancy PC?**  
A: A PC that runs Block Viewer smoothly plus hardware encode helps. Start 720p30. Gear priority in `07`.

**Q: Do I need a webcam?**  
A: No. Many SL streams are immersive-only. Webcam is optional.

---

## OBS

**Q: Game Capture vs Window Capture for Block Viewer?**  
A: Try Window Capture first. Game Capture is a hook; BV bug docs list OBS game capture among overlay hooks. See `04` + `08`. **[NEEDS CONFIRMATION]** for any official mandate.

**Q: What bitrate should I use?**  
A: High enough for clarity, low enough for your upload with headroom. Beginners often start ~2500–4500 kbps at 720p30. Test and watch network dropped frames (`01`).

**Q: MKV or MP4?**  
A: MKV is safer if OBS stops unexpectedly; remux to MP4 for editors.

---

## Kick

**Q: Where do I put the stream key?**  
A: Only in OBS (or Multistream) stream settings. Never in Discord, handbook drafts, or screenshots.

**Q: OBS says I’m live but Kick isn’t.**  
A: Wrong account, bad key/server, or delay — see `02` / `08`. Regenerate key if it may have leaked.

**Q: Can Kick ban me for multistreaming?**  
A: Check current Kick TOS / Help. Don’t assume; verify before relying on Multistream (`03`).

---

## Aitum Multistream

**Q: Should I multistream my first week?**  
A: No. Stable Kick-only first.

**Q: Why is my stream worse only when Multistream is on?**  
A: Upload or CPU saturation — drop a destination or bitrate (`03`, `08`).

---

## Block Viewer

**Q: What is Block Viewer?**  
A: An open-source Second Life client (Vulkan, GPLv3). [blockviewer.org](https://blockviewer.org/).

**Q: How do I download it?**  
A: https://blockviewer.org/download/ or GitHub Releases.

**Q: Does Block Viewer have a “Go Live” button?**  
A: No RTMP/Kick go-live inside the client. Use OBS + a platform like Kick. **Streamer mode** only cleans the on-screen UI for capture.

**Q: What is Streamer mode?**  
A: Preferences › Layout (also Me menu / toolbar): Immersive + fullscreen capture layout; hides L$ balance, world hover tips, name tags, and the mouselook Esc hint. Details: [PR #82](https://github.com/ape-squared/Block-Viewer/pull/82). Public wiki page may still be pending.

**Q: Streamer mode made OBS go black.**  
A: Fullscreen often breaks Window Capture — try Display Capture or Game Capture; see `04` / `08`.

**Q: Where are photos / posing tools?**  
A: Poser docs: https://blockviewer.org/docs/poser/

**Q: What about Blender live mesh?**  
A: LiveMesh: https://blockviewer.org/docs/livemesh/ (advanced).

**Q: How do I report a crash that only happens when streaming?**  
A: https://blockviewer.org/docs/reporting-bugs/ — mention overlays (Discord, Steam, RTSS, OBS game capture).

**Q: Is there a Discord?**  
A: Soft-open community section on the home page; join via site.

**Q: What’s the lime green color on the site?**  
A: Brand accent `#D4FF3F` — https://blockviewer.org/docs/branding/

---

## Content & graphics

**Q: How many clips should I post per stream?**  
A: 1–3 solid ones beats 10 mediocre ones (`05`).

**Q: Do I need paid overlays?**  
A: No. Start with simple full-bleed screens and OBS Text (`06`).

**Q: Can I use copyrighted music?**  
A: Risky on some platforms; prefer licensed/bed music your class approves.

---

## Audio / gear

**Q: Headphones or speakers?**  
A: Headphones — prevents echo (`07`).

**Q: My voice sounds robotic.**  
A: Ease off Noise Suppression / Gate; fix gain staging (`01`).

---

## Classes & support

**Q: What should I bring to 1:1 support?**  
A: OBS Stats screenshot, capture type, upload speed, BV version, list of overlays — **no stream keys** (`09`).

**Q: The Kick screenshots in the handbook look different.**  
A: Platform UIs change — trust current Kick Help + dashboard; report stale handbook bits.

---

## Meta FAQ for editors

**Q: Can we claim feature X for Block Viewer?**  
A: Only if present on wiki/site/docs. Otherwise **[NEEDS CONFIRMATION]** or omit.
