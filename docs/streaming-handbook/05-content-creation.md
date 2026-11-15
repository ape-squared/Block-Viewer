# 05 — Content creation: turning streams into posts (research)

**Nature of this chapter:** Mostly **practical class advice**. Cite platform help centers for export specs when freezing materials. Block Viewer contributes **source footage** (live capture, Poser stills) — it is not documented as a short-form editor.

---

## 1. Capture clips during streams

### Tools beginners should know

| Method | Where | Notes |
| --- | --- | --- |
| OBS Replay Buffer | OBS → Output → Replay Buffer | Hotkey saves last N seconds; great for “that moment” |
| Full local recording | OBS Recording | Best for longform edits; needs disk space |
| Platform clips | Kick clip tools (if available) | **[NEEDS CONFIRMATION]** current Kick clip UX / length |
| Phone screen record | Last resort | Quality/lossy; avoid for class standard |

**Practical setup:** Replay Buffer 60–120 seconds; hotkey easy to hit (e.g. F8). Announce “clip that” so mods/friends remind you.

**Why not rely only on platform VODs:** Retention policies change; quality/re-encode may be worse; offline editing is more reliable.

---

## 2. Identifying moments worth clipping

Teach a simple rubric (not motivational fluff):

**Clip if it has at least two of:**

1. Clear visual change (big build reveal, outfit swap, funny physics, scenic arrival).  
2. Clean audio line (one joke, one reaction, one explanation).  
3. Emotional beat (surprise, pride, laughter) in under 15 seconds.  
4. Understandable without the whole stream (add a one-line text hook).

**Skip:** Long silent building, internal SL drama without consent, payment/UI screens, other people’s private info.

**Block Viewer angles:** Poser results, LiveMesh before/after (advanced), pretty locations, community meetups — with consent.

---

## 3. OBS recordings & clip workflow

### Beginner pipeline

1. During stream: slap Replay Buffer on peaks.  
2. After stream: park clips in folders: `YYYY-mm-dd/raw/`.  
3. Pick 1–3 clips max for same-day posts (anti-burnout).  
4. Remux MKV → MP4 if needed (OBS → File → Remux Recording).  
5. Edit in one chosen editor (class should standardize: e.g. CapCut or DaVinci Resolve).

### File hygiene

- Name clips `topic_hook_take1.mp4`.  
- Keep raw until post is live.  
- Don’t edit the only copy.

---

## 4. Editing short-form (TikTok / Reels / Shorts)

### Shared principles

- Aspect **9:16** (1080×1920 common).  
- Safe margins: keep faces/text away from UI chrome edges.  
- Length: aim **7–45 seconds** for beginners; longer only if retention stays high.  
- First **1–2 seconds** must show the subject (hook).

### Platform notes (re-verify export presets)

| Platform | Practical notes |
| --- | --- |
| TikTok | Vertical; captions burn-in help mute viewers |
| Instagram Reels | Similar vertical; cover frame matters |
| YouTube Shorts | Vertical; title/hook text on platform too |

Link official creator docs at freeze time rather than inventing bitrate tables.

---

## 5. Basic editing principles (teachable)

1. **Cut silence** — jump cuts OK for vertical.  
2. **One idea per clip** — don’t stitch five topics.  
3. **Show then tell** — visual first, voice explains.  
4. **Zoom with purpose** — punch-in on avatar face/build detail.  
5. **Audio first pass** — if voice is bad, fix levels before fancy transitions.  
6. **Transitions:** cut > fancy wipe for beginners.

---

## 6. Captions & subtitles

**Why:** Many watch muted; accessibility; algorithm-friendly engagement signals (practical observation, not a guarantee).

**How:**

- Auto-caption in editor → **always** human-fix names (Block Viewer, SL user names).  
- High-contrast text; 2–3 lines max.  
- Prefer burned-in captions for TikTok/Reels unless platform native captions are part of class standard.

---

## 7. Hooks, pacing, retention

| Second | Job |
| --- | --- |
| 0–2 | Pattern interrupt: finished build, reaction face, bold text |
| 2–10 | Payoff or progress |
| 10–end | Button: follow for part 2 / full stream on Kick |

**Pacing drill:** Export a cut; watch on phone with sound off — do you still get it?

---

## 8. Export settings (practical starting points)

**[NEEDS CONFIRMATION]** against current editor + platform docs.

```
Resolution: 1080×1920
FPS: 30
Codec: H.264
Audio: AAC 160–192 kbps
Bitrate: editor “High” / ~8–12 Mbps for short vertical (adjust to platform caps)
```

Avoid huge ProRes masters for phone uploads unless teaching a pro track.

---

## 9. Posting consistently without burnout

**Practical schedule for class:**

- Live: 1× per week (or whatever cohort agrees).  
- Shorts: 2–3 clips from that one VOD — not 12.  
- Batch: one edit session after stream, queue posts.  
- Reuse: same clip, three platforms, slight caption tweaks — OK.  
- Off weeks: Poser stills + carousel posts instead of forced lives.

**Burnout tells:** dreading open OBS; posting zero-context drama; skipping sleep for “one more take.”

---

## 10. Building a recognizable style

Pick 3 anchors and keep them stable for 30 days:

1. **Visual:** webcam corner style OR no-cam immersive (choose).  
2. **Color:** e.g. Block lime accent `#D4FF3F` on dark panels (from branding doc) — optional.  
3. **Audio:** same intro sting length (watch copyright) or none.  
4. **Verbal:** same greeting / same “what we’re doing today” format.

Style ≠ buying a $200 overlay pack on day one.

---

## 11. Repurposing one stream → many pieces

Example from a 2-hour creative stream:

| Asset | Output |
| --- | --- |
| Arrival at location | 15s scenic hook |
| Build reveal | 30s timelapse-style jump cuts |
| Funny fail | 10s meme caption |
| Final pose (Poser) | Static post + story |
| Explainer “why Block Viewer” | 45–60s talking clip |
| Full VOD | Kick/YouTube longform if available |

**Consent:** Ask before clipping other avatars’ close-ups / voices when practical.

---

## 12. Block Viewer–specific content ideas (non-invented features)

Safe ideas tied to documented capabilities:

- Tour + candid reactions (live capture).  
- Poser photoshoot episode → stills + process clip ([Poser](https://blockviewer.org/docs/poser/)).  
- Advanced: LiveMesh WIP stream ([LiveMesh](https://blockviewer.org/docs/livemesh/)).  
- “Updating the viewer” transparency using [Updates](https://blockviewer.org/docs/updates/).  
- Tester life / bug reporting education (careful: don’t expose private crash data).

---

## 13. Screenshot / illustration needs

Items 22–24 in `00-meta-sources-gaps.md`.

---

## 14. Gaps

- Kick clip feature parity.  
- Class-standard editor choice.  
- Music copyright policy for the cohort (prefer bed music with clear license).  
