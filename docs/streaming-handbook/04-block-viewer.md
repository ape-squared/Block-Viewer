# 04 — Block Viewer for streamers (wiki-sourced research)

**This is the product-specific core of the handbook.**  
Do **not** invent features. Prefer site/wiki language. Flag gaps with **[NEEDS CONFIRMATION]**.

---

## Source index (cite these in the handbook)

| Topic | Public URL | Repo Markdown |
| --- | --- | --- |
| Home / overview | https://github.com/ape-squared/Block-Viewer/wiki/Home · https://blockviewer.org/ | `README.md`, `website/index.html` |
| Download | https://blockviewer.org/download/ | `website/download/` |
| Updates | https://blockviewer.org/docs/updates/ · [wiki Updates](https://github.com/ape-squared/Block-Viewer/wiki/Updates) | `docs/UPDATES.md` |
| Reporting bugs (incl. overlay hooks) | https://blockviewer.org/docs/reporting-bugs/ | `docs/BUG_REPORTING.md` |
| Poser (local photos) | https://blockviewer.org/docs/poser/ · wiki Poser | `docs/POSER.md` |
| LiveMesh | https://blockviewer.org/docs/livemesh/ | `docs/BLENDER_LIVE.md` |
| IK Edit | https://blockviewer.org/docs/ik-edit/ | `docs/IK_EDIT.md` |
| Animation creation | https://blockviewer.org/docs/animation-creation/ | `docs/ANIMATION_CREATION.md` |
| Branding | https://blockviewer.org/docs/branding/ | `docs/BRANDING.md` |
| Dev Toolbox | https://blockviewer.org/docs/dev-toolbox/ | `docs/DEV_TOOLBOX.md` |
| Linux install | https://blockviewer.org/docs/linux/ | `docs/LINUX.md` |
| macOS install | https://blockviewer.org/docs/macos/ | `docs/MACOS.md` |
| Testers / nameplates | https://blockviewer.org/testers/ | peers roster + site |
| Discord | Soft-open section on https://blockviewer.org/ | `website/index.html` `#discord` |
| Releases | https://github.com/ape-squared/Block-Viewer/releases | — |
| **Streamer mode** (merged feature) | No wiki page yet — verify from [PR #82](https://github.com/ape-squared/Block-Viewer/pull/82) + in-app help | `crates/block-i18n/locales/en.ftl` (`ui-options-streamer-mode*`), `crates/block-viewer/src/app.rs` |

**Important gaps:**

- There is **no** dedicated public wiki/site page titled “Streaming with OBS” as of research.
- **Streamer mode** ships in the client (PR #82, 2026-11-14) but is **not yet mirrored** on blockviewer.org docs/wiki — cite PR + in-app strings until a docs page exists.
- Capture-method advice (Window vs Game vs Display) remains OBS best practice + overlay-hook note from bug reporting — not a separate BV streaming guide.

---

## A. Getting started — what Block Viewer is

**Verified (site/README):**

- Block Viewer is an **open-source Second Life client**.
- Focus on a modern **Vulkan** renderer.
- License: **GPLv3**.
- Primary marketing/download experience: **Windows** portable builds; **macOS** and **Linux** are also documented.
- Official site: [blockviewer.org](https://blockviewer.org/).

**For streamers in plain English:**  
Block Viewer is the app you log into Second Life with. Your livestream captures that window (and your mic/webcam) in OBS and sends it to Kick. Block Viewer is **not** the streaming platform and does **not** replace OBS.

**What it is not (do not claim):**

- Not a built-in RTMP encoder / Kick client — you still need OBS (or similar) + a platform.  
- Not documented as shipping a clip editor or VOD host.  
- Not a multistream tool (that’s Aitum / similar).  
- **Does** include a **Streamer mode** UI layout for clean capture (see §E0) — do not confuse that with “streaming to Kick from inside BV.”

---

## B. Installation

### B1. Windows (typical class path)

**Verified pattern from download/site messaging:**

1. Open https://blockviewer.org/download/ (or GitHub Releases).
2. Download the current Windows zip / portable build listed there.
3. Unzip to a folder you control (e.g. `Downloads\BlockViewer` or `C:\Games\BlockViewer`).
4. Run the viewer executable from the unzipped folder.
5. Log in with Second Life account credentials.

**Practical tips:**

- Prefer paths without exotic permissions issues.
- Allow GPU drivers to be reasonably current (Vulkan).
- First launch: expect normal SL-style login (username/password or whatever the current build presents — don’t invent SSO).

**SHOT:** download page; zip contents; login screen.

### B2. macOS / Linux

Point students to:

- https://blockviewer.org/docs/macos/
- https://blockviewer.org/docs/linux/

Class may be Windows-first; don’t pretend parity of every feature without reading those pages.

### B3. Staying updated

- https://blockviewer.org/docs/updates/
- Teach: update from official download/Releases; don’t sideload random “repacks.”

---

## C. First-time configuration (streamer-oriented)

*In-viewer preferences are large; only document what wiki/docs actually say. Graphics “best for streaming” presets are **[NEEDS CONFIRMATION]**.*

### Practical first-run checklist (label as class advice)

1. Log in successfully offline-of-stream first.
2. Learn where **Streamer mode** lives (Preferences › Layout, Me menu, toolbar) — see §E0. Practice toggling it **before** go-live.
3. For capture experiments without Streamer mode: **windowed** or **borderless** is often easier for OBS Window Capture than exclusive fullscreen — practical advice.
4. Disable or limit other overlays (Discord overlay, Steam overlay, RTSS) while testing capture — aligns with bug doc awareness of overlay hooks.
5. Note your GPU (NVIDIA/AMD/Intel) for OBS encoder choice.
6. Join class Discord / community channels for help.

**Capture note when Streamer mode is on:** It enters fullscreen (borderless fill of the monitor per PR #82). Window Capture can be unreliable in true fullscreen; students should test **Display Capture** of that monitor and/or **Game Capture**, and keep the privacy habit (no passwords on that display). Re-test after each OBS/BV update.

### Branding awareness (optional for on-stream overlays)

From https://blockviewer.org/docs/branding/ / `docs/BRANDING.md`:

- Accent lime: `#D4FF3F`
- Dark charcoal surfaces in site language

Useful if students make “Block Viewer class” overlays — not required to run the client.

---

## D. How Block Viewer fits a streaming workflow

```
Second Life account
        ↓
Block Viewer (render world, chat, build, pose)
        ↓
OBS (Window/Game/Display Capture + mic + cam + scenes)
        ↓
Kick (and optional Aitum Multistream → other platforms)
        ↓
Clips / local recordings → short-form content
```

**Creator loop with BV tools:**

| Goal | BV-related path | Docs |
| --- | --- | --- |
| Clean fullscreen look for OBS/Kick | **Streamer mode** | PR #82 + in-app help (wiki pending) |
| Live hangout / explore / build on stream | Capture BV in OBS | OBS KB + this section |
| Hide L$ on screen share (without full Streamer mode) | Preferences show-balance / balance context menu | `en.ftl` `ui-options-show-balance*` |
| Still photos / portraits for thumbnails & posts | **Poser** | `/docs/poser/` |
| Custom mesh iteration with Blender | **LiveMesh** | `/docs/livemesh/` |
| Animation work (advanced) | **IK Edit** / **Animation creation** | `/docs/ik-edit/`, `/docs/animation-creation/` |
| Report crashes (esp. with overlays) | Bug reporting + logs | `/docs/reporting-bugs/` |
| Show you’re a tester | Nameplate signup | `/testers/` |

---

## E. Feature walkthroughs (streamer-relevant)

### E0. Streamer mode (verified from client + PR #82)

**Status:** Shipped in client via [PR #82](https://github.com/ape-squared/Block-Viewer/pull/82) (merged 2026-11-14). **No blockviewer.org / wiki article yet** — handbook must cite PR + UI strings until docs catch up. **[NEEDS CONFIRMATION]** when a public docs URL is published; update this section’s links.

**In-app help text** (`ui-options-streamer-mode-help` in `crates/block-i18n/locales/en.ftl`):

> Fullscreen capture layout for creators: enters Immersive mode (toolbars hidden, chat kept), fills the monitor, and hides the L$ balance, world hover tips, name tags, and the mouselook Esc hint. Your other preferences stay as they are; turn this off or leave Immersive to restore the usual interface. Also on Me › Streamer mode and as a toolbar button.

**How to turn it on (verified UI paths from PR + code):**

1. **Preferences › Layout › Streamer mode**, or  
2. **Me › Streamer mode**, or  
3. Toolbar button `streamer_mode` (“Streamer mode”).

**What it does (verified):**

| Behavior | Detail |
| --- | --- |
| Immersive chrome | Toolbars hidden; chat kept (per help string) |
| Fullscreen | Fills the monitor; if it had to force fullscreen on, turning Streamer mode off restores windowed |
| Hides while on | L$ balance, world hover tips, name tags, mouselook Esc hint |
| Other prefs | Not permanently rewritten — suppressed while Streamer mode is on |
| Leave Immersive | Leaving Immersive also ends Streamer mode |
| Persistence | Setting is saved; on restart in-world, immersive chrome is re-applied if Streamer mode is still on |

**Related (can use without full Streamer mode):**

- **Show the L$ balance** preference / balance context menu — help text explicitly mentions hiding balance while streaming or screen sharing (`ui-options-show-balance-help`).

**Class tutorial angle:**

1. Arrive at location windowed; set up OBS.  
2. Enable Streamer mode → confirm clean look.  
3. Confirm OBS still captures (Display/Game/Window — test).  
4. Go live.  
5. Disable Streamer mode or leave Immersive when done → UI returns.

**Common mistakes:**

- Expecting Streamer mode to “go live on Kick” by itself.  
- Forgetting it fullscreen’d the monitor and accidentally Display-Capturing a second monitor that shows Discord with keys.  
- Fighting Window Capture black frames while fullscreen — switch capture type.

**SHOT needs:** Preferences › Layout toggle; Me menu entry; before/after immersive chrome; toolbar button lit.

### E1. Everyday live session

1. Launch Block Viewer; log in.  
2. Arrive at stream location; hide sensitive IMs if needed.  
3. Enable **Streamer mode** when ready for a clean frame (optional but recommended once capture works).  
4. OBS Live scene capturing BV + webcam + mic.  
5. Kick go-live.  
6. Interact with SL chat **and** Kick chat (two chats — teach cognitive load).  
7. Ending scene → stop stream → turn Streamer mode off → optionally save poses/photos.

### E2. Poser — content between or during streams

**Source:** https://blockviewer.org/docs/poser/ · `docs/POSER.md`

Teach from the actual Poser doc (do not paraphrase features that aren’t there). High-level handbook angle:

- Local posing / photo workflow for avatars.
- Useful for: channel art, TikTok stills, “outfit of the day,” thumbnail frames.
- Stream angle: show Poser on stream as a creative segment, or use offline for assets.

**Editor task:** Pull step lists **verbatim in spirit** from current Poser page when drafting the final chapter.

### E3. LiveMesh — advanced creative streams

**Source:** https://blockviewer.org/docs/livemesh/

For creators who model in Blender and want live mesh workflows. Handbook should:

- Mark as **advanced**.
- Link the wiki/site page.
- Not required for “first livestream” path.

### E4. IK Edit & animation creation — advanced

**Sources:** `/docs/ik-edit/`, `/docs/animation-creation/`

Same rule: link + summarize only what those pages state. Good “level 2 class” material.

### E5. Dev Toolbox — diagnostics, not a stream toy

**Source:** `/docs/dev-toolbox/`

Mention for troubleshooting class: diagnostics when things break — not an overlay for viewers unless a student knowingly demos it.

### E6. Overlay hooks & capture (critical accuracy)

From `docs/BUG_REPORTING.md` (Reporting bugs on site):

Bug reports can note **Overlay hooks**: programs that injected hooks into the viewer — including **Discord, Steam, RivaTuner, OBS game capture** — from a fixed list.

**Handbook implications (honest wording):**

- OBS **Game Capture** is a hook-based capture method.
- If students see crashes, freezes, or weird startup issues while streaming, ask whether Discord overlay / Steam overlay / RTSS / Game Capture were active.
- Prefer trying **Window Capture** or **Display Capture** as a reliability test.
- Do **not** claim Game Capture is officially unsupported unless maintainers say so — claim: “known as an overlay hook; test carefully; include it when filing bugs.”

---

## F. Recommended settings (split verified vs practical)

### Verified

- Use official builds from blockviewer.org / GitHub Releases.
- Follow platform install docs for macOS/Linux.
- When filing bugs after stream issues, report overlay hooks honestly.
- Use **Streamer mode** for a clean fullscreen capture layout (PR #82 / in-app help).
- Hide L$ via show-balance preference or Streamer mode when screen sharing.

### Practical (class) — **[NEEDS CONFIRMATION]** for any that become “official”

| Setting / habit | Suggestion | Why |
| --- | --- | --- |
| Streamer mode | On for live segment once OBS capture is proven | Clean frame without hunting every chrome toggle |
| Without Streamer mode | Borderless/windowed | Friendlier to Window Capture |
| With Streamer mode on | Test Display Capture / Game Capture | Fullscreen may break Window Capture |
| Particle / draw distance | Balance pretty vs FPS | Encoding overload |
| Discord overlay | Off while testing | Fewer hooks |
| Second monitor | OBS + Kick chat on side monitor | Don’t capture secrets / keys |

---

## G. Step-by-step tutorials to include in final handbook

### Tutorial 1 — First Block Viewer capture on OBS (beginner)

1. Install OBS (`01-obs-studio.md`).  
2. Install Block Viewer (section B).  
3. Log into BV; stay windowed/borderless first.  
4. OBS → Live scene → Window Capture → select Block Viewer.  
5. Audio: Desktop + Mic.  
6. Local record 60 seconds; verify playback.  
7. Turn on **Streamer mode**; re-check capture (switch to Display/Game Capture if black).  
8. Only then connect Kick.

### Tutorial 2 — Soft open to Kick with Starting Soon

1. Scenes: Starting Soon / Live / BRB / Ending.  
2. Kick title mentions Block Viewer / Second Life honestly.  
3. Start stream on Starting Soon; enable Streamer mode; switch to Live; greet.  
4. Ending scene → stop stream → leave Streamer mode / Immersive.

### Tutorial 3 — Poser stills for thumbnails

1. Follow current Poser doc steps.  
2. Export/save per doc.  
3. Use still in Kick offline banner or short-form cover.

### Tutorial 4 — Reporting a stream-related crash

1. Note overlays active (Discord, OBS Game Capture, etc.).  
2. Follow https://blockviewer.org/docs/reporting-bugs/.  
3. Bring logs to class Discord / 1:1 support.

---

## H. Troubleshooting (Block Viewer–specific)

| Symptoms | Likely causes | Fixes / next steps |
| --- | --- | --- |
| Black capture in OBS | Wrong source; minimized window; fullscreen/Streamer mode; hook conflict | Match capture to window mode; Display Capture while Streamer mode on; update GPU drivers |
| Streamer mode “does nothing” / UI still busy | Not in world; outdated build pre-#82 | Update from official download; toggle Me › Streamer mode |
| Left Immersive and lost Streamer look | By design — leaving Immersive ends Streamer mode | Re-enable Streamer mode |
| BV crash when going live | Overlay hooks (Discord/Steam/RTSS/Game Capture) | Disable overlays; switch capture type; file bug with hooks listed |
| Low FPS on stream | High in-world settings + encode on same GPU | Lower BV graphics; use NVENC; 720p30 |
| Login fails | Creds, grid status, outdated build | Reset SL password; update BV; check status outside handbook scope |
| Audio from world missing | Desktop audio device mismatch | Windows default device; OBS Audio settings |
| “I can’t find Go Live inside BV” | Expected — Streamer mode ≠ broadcasting | Use OBS + Kick; Streamer mode only cleans the UI |

More matrix: `08-troubleshooting.md`.

---

## I. Best practices for BV streamers (practical)

1. **Protect privacy** — SL IMs, payment UI, email: don’t Display Capture carelessly.  
2. **Two chats** — assign a mod or slow down.  
3. **Credit the client** — “Streaming in Block Viewer” in title/panels helps the project and sets expectations.  
4. **Official links only** — blockviewer.org, GitHub Releases.  
5. **Community** — Discord soft-open; tester signup if they’re testing builds.  
6. **Separate creative days** — Poser/LiveMesh deep work vs chatty live hangouts.  
7. **Update cadence** — read Updates page; don’t update mid-stream.

---

## J. FAQ seeds (BV)

See also `11-faqs.md`.

- **Is Block Viewer free?** Open-source GPLv3 client — follow site for download; Second Life itself has its own account rules.  
- **Does BV stream to Kick?** No — use OBS (or similar). Streamer mode only prepares a clean capture layout.  
- **What is Streamer mode?** Immersive + fullscreen capture layout that hides balance, hover tips, name tags, and mouselook Esc hint (PR #82).  
- **Windows only?** Site advertises Windows + macOS; Linux docs exist — check download page for current artifacts.  
- **Can I use Firestorm and BV interchangeably on stream?** Different clients; handbook focuses on BV.  
- **Where do I get help?** Class support + Discord + bug reporting docs.

---

## K. Screenshot needs

`00-meta-sources-gaps.md` items 15–21.

---

## L. Gaps requiring maintainer / instructor confirmation

| ID | Question |
| --- | --- |
| G1 | Official recommended OBS capture mode for Vulkan BV (esp. with Streamer mode fullscreen)? |
| G2 | Recommended graphics prefs for 720p/1080p streams? |
| G5 | Any known bad interactions with specific OBS versions? |
| G6 | Class OS support matrix (Windows-only vs multi)? |
| G7 | Confirm no first-party clip/VOD tool — still use OBS replay buffer |
| G9 | Publish Streamer mode on wiki/site docs and replace PR #82 citations |
| — | Any upcoming “creator” features that would obsolete parts of this chapter? |

---

## M. Suggested final-chapter outline (for Claude)

1. Getting started — what BV is / isn’t  
2. Installation  
3. First-time configuration  
4. **Streamer mode** (hero product feature for this handbook)  
5. Feature walkthroughs (live session, Poser, advanced tools)  
6. Advanced workflows (LiveMesh, animation)  
7. Troubleshooting  
8. FAQ  
9. Link box to all wiki/site pages above (+ PR #82 until wiki exists)  
