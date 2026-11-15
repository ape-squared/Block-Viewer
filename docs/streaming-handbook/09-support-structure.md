# 09 — Support structure: handbook + live classes

**Nature:** Framing for the free streaming & content-creation classes that accompany this handbook. Adjust names/times to the actual cohort schedule when publishing.

---

## 1. How the pieces fit

| Piece | Role |
| --- | --- |
| This handbook | Self-serve reference: steps, why, troubleshooting |
| Group sessions | Live demos, shared Q&A, accountability |
| One-on-one support | Machine-specific capture/audio/debug |
| Community (Discord soft-open on [blockviewer.org](https://blockviewer.org/)) | Peer help between sessions |
| Official docs | Source of truth for BV / OBS / Kick / Aitum |

**Rule:** Handbook first → then ask. Bring evidence (screenshots, not vibes).

---

## 2. Using the handbook alongside live support

### Before a group session

1. Skim the chapter for that week (e.g. OBS scenes).  
2. Attempt the numbered steps once.  
3. Note where you got stuck **with step numbers**.  
4. Prepare a 30-second summary: goal → what you tried → what you saw.

### During session

- Screenshare OBS + BV (blur stream keys).  
- Prefer “watch me click” over abstract debate.  
- Capture the fix in your personal notes / update your go-live checklist.

### After session

- Re-run the flow alone the same day.  
- Post a short “resolved: black screen was Game Capture on Vulkan — Window Capture works” tip for peers (optional).

---

## 3. What questions to bring (high-signal)

**Good:**

- “Window Capture of Block Viewer is black when I Alt-Tab — step 5 of Tutorial 1.”  
- “Kick shows live but Desktop Audio meter is flat; Windows default is Headphones.”  
- “NVENC missing in OBS — GPU is Intel-only laptop.”  
- “Crash when enabling Game Capture; Discord overlay was on — should I file a BV bug?”

**Weak (workshop will re-ask anyway):**

- “It doesn’t work.”  
- Pasting stream keys.  
- “Make me viral.”  

---

## 4. Collaborative troubleshooting protocol

1. **Reproduce** with local recording before blaming Kick.  
2. **Isolate** — change one variable (capture type XOR bitrate XOR overlays).  
3. **Evidence** — OBS Stats, device names, BV version, overlay list.  
4. **Sanitize** — redact keys, emails, other users’ private IMs.  
5. **Escalate** — class helper → BV bug report if client crash ([Reporting bugs](https://blockviewer.org/docs/reporting-bugs/)).

---

## 5. One-on-one sessions — best uses

- Laptop multi-GPU capture hell  
- Accessibility needs (audio monitoring, UI scale)  
- First-ever go-live nerves (shoulder-surf checklist)  
- Multistream only after single-stream success  

Not ideal for: designing full overlay brands from scratch (point to `06` + DIY).

---

## 6. Community norms (teach explicitly)

- No sharing stream keys or passwords.  
- Consent before clipping others.  
- Assume beginners in chat — no gatekeeping.  
- Official downloads only (blockviewer.org / OBS / Aitum / Kick).  
- Report handbook mistakes (stale Kick UI) so materials update.

---

## 7. Suggested cohort milestones

1. Local OBS record of BV (not live).  
2. First Kick live ≤30 minutes with Starting Soon/Ending.  
3. First three clips posted.  
4. Optional: Multistream lab.  
5. Optional: Poser thumbnail workshop.

Handbook chapters map 1→`01`/`04`, 2→`02`, 3→`05`, 4→`03`, 5→`04` Poser.
