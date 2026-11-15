# 07 — Audio, lighting & equipment (budget-first research)

**Principle:** Prioritize **mic intelligibility** and **stable capture** before RGB lights and 4K cams. All recommendations below are **practical class advice**, not Block Viewer product requirements.

---

## 1. What beginners should buy first (priority order)

| Priority | Item | Why |
| --- | --- | --- |
| 1 | Decent USB headset **or** entry USB mic + headphones | Viewers forgive video before unintelligible audio |
| 2 | Wired Ethernet (or strong 5 GHz Wi‑Fi) | Stops mysterious “Kick lag” |
| 3 | Working webcam you already own | Facecam optional for SL; nice-to-have |
| 4 | Cheap key light / desk lamp with soft diffusion | Face visibility if using cam |
| 5 | Second monitor | Chat + OBS without Display Capturing secrets |
| Later | XLR interface, ring light kits, capture cards | After consistency |

**Can wait:** Elgato-tier lights, shotgun mics, greenscreen, stream decks, multistream paid tiers.

---

## 2. Microphone positioning

### Headset mic

- Mic boom near corner of mouth, **not** in the breath stream.  
- 1–2 finger gaps from lips.  
- Foam windscreen if plosives (“P”, “B”) pop.

### Desktop USB mic

- 15–25 cm from mouth.  
- Slightly off-axis to reduce plosives.  
- Shock mount / soft surface to reduce desk thumps.  
- **Headphones required** — speakers + mic = echo.

### Levels (tie to OBS)

- Windows mic level ~50–80%; avoid maxed OS + maxed OBS gain.  
- OBS peaks −12 to −6 dB speaking.  
- Filters: see `01-obs-studio.md`.

---

## 3. Preventing clipping & harsh audio

| Problem | Fix |
| --- | --- |
| Red meters constantly | Lower Windows or OBS fader; add Limiter last |
| Distorted but meters OK | Hardware gain too high / bad USB power |
| Muddy voice | Move closer; reduce Noise Suppression aggression; treat room a bit |
| Keyboard loud | Gate lightly; move mic; soft desk mat |

**Clipping** = tops of waveform chopped → crackle. Prevention > “fix it in post” for live.

---

## 4. Room noise

Practical cheap treatments:

- Close windows; move PC fan exhaust away from mic.  
- Blanket behind mic (improvised absorption).  
- Mute notifications; phone on DND.  
- In BV: lower UI/sounds that stack with music beds.

**Noise Suppression** helps; it won’t save a blender running next to the mic.

---

## 5. Affordable lighting (facecam)

### Goals

- Face brighter than the monitor glow behind you.  
- Soft light (lamp through parchment / cheap softbox / bounce off wall).  
- Avoid single overhead “horror” lighting.

### Budget setups

1. **Desk lamp + white wall bounce** — $0–20.  
2. **One LED panel / ring light** at 45° — entry kits.  
3. **Key + dim fill** — second cheaper lamp opposite.

**SL-only immersive streams:** Lighting can wait; spend effort on in-world look + OBS capture.

---

## 6. Webcam positioning

- Lens at **eye height**, not looking up nostrils.  
- Arm’s length; then crop in OBS.  
- Background: tidy, or soft bokeh if cam supports; no sensitive documents.  
- 720p30 is fine for corner cam.

**Privacy:** Disable cam on BRB if leaving the desk.

---

## 7. Budget gear examples (illustrative, not endorsements)

| Tier | Audio | Video | Light |
| --- | --- | --- | --- |
| Already own | Phone earbuds with mic (temporary) | Built-in laptop cam | Window daylight |
| Starter | USB headset with boom | 1080p USB webcam | Desk lamp |
| Next step | USB condenser + headphones | Same cam | One soft LED |

**Update prices/models at class time** — don’t hardcode SKUs that go EOL.

---

## 8. PC / encode gear reality

- **GPU with NVENC / AMF** makes streaming BV + encode easier.  
- Capture cards are for consoles — usually **irrelevant** for Block Viewer on the same PC.  
- More RAM helps SL clients generally; exact BV requirements → check current download/release notes **[NEEDS CONFIRMATION]** if handbook cites minimums.

---

## 9. Common mistakes

- Speakers instead of headphones.  
- RGB mic with terrible gain staging.  
- Buying cam before fixing Ethernet.  
- Ring light reflection on glasses (angle light higher/side).  
- Assuming expensive = necessary for first Kick stream.

---

## 10. Class exercise

1. Record 30s talking head or talking-over-BV.  
2. Peer review: “Could you understand every word on phone speaker?”  
3. Only then discuss lights/overlays.
