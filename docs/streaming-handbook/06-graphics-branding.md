# 06 — Stream graphics & branding (research)

**Nature:** Practical production guidance + optional Block Viewer brand tokens from [Branding](https://blockviewer.org/docs/branding/) (`docs/BRANDING.md`). Not a substitute for a designer course.

---

## 1. Overlays (plain English)

An **overlay** is usually a transparent PNG (or web page) drawn on top of your game/viewer capture: webcam frame, corner logo, thin bar with socials.

**Job:** Identity + readability.  
**Not the job:** Cover the interesting part of Second Life.

### Beginner rules

1. One overlay set per brand — don’t swap every stream.  
2. Leave **>70%** of the frame clean for BV content (practical).  
3. No tiny unreadable text.  
4. No fake “follower goal” widgets until chat actually exists.  
5. Match webcam frame thickness to 1080p readability.

---

## 2. Alerts

**Alerts** = on-screen animation/sound when someone follows, subscribes, donates, etc. (via Browser Source + alert service).

**Practical day-one advice:** Skip alerts until Kick + OBS are stable. Alerts add Browser Source complexity and surprise audio.

When added later:

- Keep duration short (2–5s).  
- Volume below voice.  
- Test in Starting Soon, not mid-collab.

---

## 3. Starting Soon / BRB / Ending screens

Full-bleed images or short loops — **one message each**.

| Screen | Must include | Avoid |
| --- | --- | --- |
| Starting Soon | Show title / “starting soon” | Schedules, stats, address blocks, clutter |
| BRB | “BRB” + optional ETA vibe | Essay of excuses |
| Ending | Thanks + where to follow (Kick, Discord) | Five competing CTAs |

Align with class design rules: brand-readable, not dashboard soup. Optional accent `#D4FF3F` on dark charcoal for Block-affiliated classes.

**SHOT:** Three example full-bleed layouts.

---

## 4. Webcam frames & layouts

### Common Live layouts

1. **Immersive** — BV full frame; no cam (great for scenic).  
2. **Corner cam** — 16:9 or circular cam bottom-right; don’t cover chat-critical UI.  
3. **Facecam focus** — rare for SL; only for talking-head segments.

**Practical:** For building streams, corner cam small. For storytelling, larger cam during talking segments (scene switch).

---

## 5. Branding consistency

From Block Viewer branding doc (optional for student personal brands):

- Lime accent `#D4FF3F`  
- Dark surfaces  

**Personal brand checklist:**

- Same display name spelling everywhere.  
- Same avatar portrait style for profile vs offline banner.  
- Same fonts in overlays (pick one display + one body; avoid default Inter-looking generic stacks if designing from scratch — class preference).  
- Logo that works at 48px height.

---

## 6. Importing graphics into OBS

### Image overlay

1. Create PNG with transparency (Photoshop, Photopea, GIMP, Canva — with transparent export).  
2. OBS scene → + → **Image** → select PNG.  
3. Resize with transform; hold Shift for aspect if needed.  
4. Layer order: capture at bottom, overlay above, cam above/beside carefully.

### Media / slideshow

- Media Source for animated webm/mp4 backgrounds (watch CPU).  
- Image Slide Show for rotating tips on BRB.

### Browser Source

- Alert providers / HTML overlays. Advanced; need local or hosted URL.

---

## 7. Avoiding clutter

**Kill these on day one:**

- Multi-stat tickers  
- Dual chat overlays + Kick chat + SL chat all visible  
- Animated particles over the whole screen  
- Giant “LIVE” stickers covering the avatar face  
- Unreadable 8pt social handles

**Readable test:** Shrink preview to phone width. If you can’t tell what’s happening in BV, simplify.

---

## 8. DIY without a $99 pack

1. Solid dark full-bleed BRB with one accent line.  
2. Thin L-shaped webcam border.  
3. Text source in OBS for “Starting soon” until custom art exists.  
4. Poser screenshot ([Poser docs](https://blockviewer.org/docs/poser/)) as offline Kick banner.

---

## 9. Common mistakes

- Buying huge overlay packs before first successful stream.  
- Opaque overlays hiding gameplay.  
- Copyrighted anime/game art as “brand” without rights.  
- Different aspect canvases (canvas 1080p, art made at random size → blurry).

**Fix:** Design overlays at **1920×1080** to match OBS canvas.

---

## 10. Screenshot needs

Items 22–23 in meta doc.
