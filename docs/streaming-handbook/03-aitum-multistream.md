# 03 — Aitum Multistream (research)

**Official sources to re-open before class freeze:**

- Product: https://aitum.tv/products/multistream  
- Docs hub: https://docs.aitum.tv/ (confirm current paths)  
- Download / OBS plugin instructions: follow links from the product page

**[NEEDS CONFIRMATION]** Exact UI labels, supported destinations list, and Kick setup steps — verify against current Aitum docs; do not invent menu paths.

---

## 1. What Aitum Multistream is

**Aitum Multistream** is a tool (commonly used as an OBS-oriented workflow/plugin ecosystem from Aitum) that lets you send **one** encoded (or managed) stream out to **multiple** platforms/destinations — e.g. Kick + Twitch + YouTube — without running three separate OBS instances.

Plain English: You produce the show once; Multistream fans it out to several sites.

---

## 2. Why creators multistream

| Reason | Notes |
| --- | --- |
| Reach | Different audiences live on different platforms |
| Backup discovery | Growing on Kick while keeping YouTube presence |
| Community preference | Friends split across apps |

**Tradeoffs:**

- More chat surfaces to moderate.
- Some platforms discourage or limit simultaneous streaming — **check each platform’s TOS** (**[NEEDS CONFIRMATION]** per platform).
- Upload bandwidth and CPU/GPU load increase (especially if re-encoding per destination).
- Platform-specific features (Kick-only raids, YouTube Super Chat UX) still need attention per site.

---

## 3. Installation & setup (pattern)

*Replace with exact clicks from current Aitum docs.*

### Typical beginner path (research outline)

1. Create / log into Aitum account if required by current product flow.
2. Download Multistream from official Aitum site (not random mirrors).
3. Install per docs (OBS plugin and/or companion app — **confirm which model is current**).
4. Restart OBS if instructed.
5. Open Multistream settings from the documented OBS dock/menu.
6. Authenticate or paste destinations.

**Common mistake:** Installing outdated plugin builds from third-party “mod packs.”

---

## 4. Configuring multiple destinations

### Concepts

- **Destination** — One platform endpoint (server + key or OAuth).
- **Primary vs extra** — Some setups stream once to a relay that forks; others may re-encode. Performance differs — read Aitum’s current architecture notes.

### Steps (generic)

1. Add destination → choose platform (Kick, etc.).
2. Prefer **OAuth/connect** when offered (less key mishandling).
3. If key-based: paste server + key (same hygiene as `02-kick.md`).
4. Enable destinations you want for this session.
5. Apply → start stream from OBS/Multistream as documented.

**Kick:** Use the same ingest/key rules as single-platform; never paste keys into chat.

**SHOT:** Destination list with keys redacted.

---

## 5. Encoder settings & performance

### What to teach

1. **One good encode** beats three bad ones. Stabilize single-platform first.
2. Multistream may need **higher upload** (rough practical rule: sum of bitrates or relay overhead — confirm with Aitum docs whether destinations share one encode).
3. Watch OBS Stats: **dropped frames (network)** and **encoding lag**.
4. If overloaded: drop a destination, lower resolution/FPS/bitrate, use hardware encoder.

### Practical beginner recommendation

- Week 1–2: Kick only.
- When stable: add **one** extra destination.
- Don’t enable five platforms on a laptop hotspot.

---

## 6. Multistream vs multiple OBS instances

| Approach | Pros | Cons |
| --- | --- | --- |
| Aitum Multistream | One scenes/audio setup; designed for multi-dest | Learning curve; depends on plugin health |
| Two OBS instances | Isolation | Double CPU, easy to desync scenes, nightmare audio |
| Platform built-in restream (if any) | Simple | Feature limits; TOS |

**Class recommendation:** Multistream tool **or** single platform — not dual OBS for beginners.

---

## 7. Limitations & troubleshooting

| Issue | Likely cause | Direction |
| --- | --- | --- |
| One platform up, other down | Bad key, platform outage, destination disabled | Re-auth that destination only |
| All destinations fail | OBS not started correctly, Multistream not armed | Follow Aitum start-order docs |
| Severe lag only when multi | Upload saturation | Lower bitrate; wired net; fewer dests |
| Plugin missing after OBS update | Version skew | Reinstall matching Multistream build |
| Audio OK locally, missing on one site | Track/bitrate per dest misconfig | Check per-destination audio settings if exposed |

**Prevention:** Update OBS + Multistream together; keep a written “go-live order” card.

---

## 8. Recommended beginner workflow

1. Master OBS → Kick alone (`01`, `02`).
2. Install Aitum Multistream from official docs.
3. Add Kick as destination #1; confirm parity with previous setup.
4. Add second destination; test privately.
5. Moderation plan: open both chats or use a chat aggregator later (advanced).
6. Keep Block Viewer capture settings unchanged while testing Multistream (isolate variables).

---

## 9. Gaps

- Current free vs paid tiers / destination limits.
- Exact Kick OAuth vs key flow in latest Multistream.
- Whether Aitum re-encodes per destination (critical for CPU advice).
- Compatibility matrix vs OBS version used in class.

---

## 10. What Claude should emphasize

- Multistream is optional power-up, not day-one required.
- Bandwidth math and TOS honesty.
- Key safety identical to single-stream Kick.
