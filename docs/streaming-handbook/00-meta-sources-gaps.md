# Meta: sources, gaps, screenshot & illustration needs

## Citation policy for the handbook

- **Verified** — paraphrased from official docs or Block Viewer wiki/site; include URL.
- **Practical advice** — class/instructor recommendation; label as such.
- **[NEEDS CONFIRMATION]** — missing docs, UI that changes often, or product behavior not documented.
- **Never** paste real stream keys, passwords, Discord tokens, or GitHub PATs.

---

## Primary official sources (non–Block Viewer)

### OBS Studio

| Resource | URL | Use for |
| --- | --- | --- |
| OBS homepage / download | https://obsproject.com/ | Install links |
| OBS Quickstart | https://obsproject.com/kb/quickstart | First launch, scenes/sources |
| OBS Studio Quickstart (docs) | https://obsproject.com/wiki/OBS-Studio-Quickstart | Alternate quickstart |
| Sources guide | https://obsproject.com/kb/sources-guide | Capture types |
| Game Capture | https://obsproject.com/kb/game-capture-source | Vulkan/DirectX games |
| Display Capture | https://obsproject.com/kb/display-capture-source | Full screen |
| Window Capture | https://obsproject.com/kb/window-capture-source | Single window |
| Audio Mixer | https://obsproject.com/kb/audio-mixer-guide | Levels, monitoring |
| Filters | https://obsproject.com/kb/filters-guide | Noise suppression, compressor, etc. |
| Streaming settings | https://obsproject.com/kb/streaming-settings | Bitrate / encoder |
| Recording | https://obsproject.com/kb/recording-settings | Local record |
| Troubleshooting | https://obsproject.com/kb/troubleshooting | Black screen, etc. |
| Auto-Configuration Wizard | https://obsproject.com/kb/auto-configuration-wizard | First-time wizard |

### Kick

| Resource | URL | Use for |
| --- | --- | --- |
| Kick Help Center | https://help.kick.com/ | Account, creator, streaming |
| Stream key / OBS setup | Search Help for “stream key”, “OBS”, “encoder” | Connecting OBS |
| Creator Dashboard | In-product (creator.kick.com or dashboard after login) | Titles, categories — **UI drifts; screenshot often** |
| Community / moderation | Help articles on chat, mods, bans | Community section |

**[NEEDS CONFIRMATION]** Exact Help Center article slugs change. Before class
freeze, open https://help.kick.com/ and capture current “Getting Started /
Streaming with OBS / Stream Key” article URLs.

### Aitum Multistream

| Resource | URL | Use for |
| --- | --- | --- |
| Aitum Multistream product | https://aitum.tv/products/multistream | What it is |
| Aitum docs / setup | https://docs.aitum.tv/ (or linked from product page) | Install, destinations |
| OBS plugin / download | Follow current Aitum download page | Version pin |

**[NEEDS CONFIRMATION]** Aitum docs site structure and Kick destination steps —
re-open product docs the week of class.

### Short-form / editing (optional citations)

| Resource | Notes |
| --- | --- |
| CapCut / DaVinci Resolve / Premiere Express help | Prefer one free editor for class consistency |
| Platform creator guides: TikTok, Instagram Reels, YouTube Shorts | Export aspect 9:16, captions |

---

## Block Viewer — verified product facts (do not invent beyond this)

Pulled from site + wiki + repo docs (see `04-block-viewer.md` for detail):

1. **What it is:** Open-source Second Life client; Vulkan renderer; GPLv3.
2. **Platforms:** Windows (primary portable zip), macOS and Linux documented.
3. **Download:** https://blockviewer.org/download/ and GitHub Releases.
4. **Portable Windows build:** unzip, run — no heavy installer narrative on site.
5. **Login:** Second Life credentials (standard SL login flow).
6. **Streamer mode:** Client feature (PR #82) — immersive + fullscreen capture layout; hides L$ balance, hover tips, name tags, mouselook Esc hint; Me menu / Preferences › Layout / toolbar. **Not yet on public wiki** — cite PR + `en.ftl` until docs publish.
7. **Show L$ balance:** Preference/help text mentions hiding while streaming or screen sharing.
8. **Poser:** Local photo / posing workflow documented at `/docs/poser/`.
9. **LiveMesh:** Blender ↔ viewer mesh workflow at `/docs/livemesh/`.
10. **IK Edit / Animation creation:** Advanced avatar animation docs exist.
11. **Bug reports:** Overlay hooks (Discord, Steam, RivaTuner, **OBS game capture**) are explicitly listed as injectors that can affect sessions — see `docs/BUG_REPORTING.md`.
12. **Branding:** Lime `#D4FF3F`, dark charcoal surfaces — `docs/BRANDING.md`.
13. **Community:** Discord soft-open on home page; tester nameplate signup at `/testers/`.
14. **No dedicated “streaming with OBS” wiki page** as of research date — capture guidance must be labeled practical + OBS docs, not BV wiki.

---

## Gaps & uncertainties (flag in handbook)

| ID | Gap | Suggested handbook treatment |
| --- | --- | --- |
| G1 | No official Block Viewer “OBS capture” page | Practical: Window vs Display vs Game Capture; cite OBS KB; note Vulkan + overlay hooks; Streamer mode fullscreen implications |
| G2 | Recommended in-world graphics presets for streamers | **[NEEDS CONFIRMATION]** from maintainers / class instructors |
| G3 | Kick Help article URLs / dashboard paths | Re-verify before publish; use screenshots |
| G4 | Aitum Multistream version + Kick destination UI | Re-verify against docs.aitum.tv |
| G5 | Whether Game Capture hooks cause BV instability | Bug doc lists OBS game capture as overlay hook — test carefully; do not invent a hard ban |
| G6 | macOS/Linux streaming path for BV classes | Class may be Windows-first; point to platform docs |
| G7 | Built-in clip / VOD tools inside Block Viewer | **None documented** — do not invent; use OBS replay buffer / Kick clips |
| G8 | Official “creator dashboard” / RTMP inside Block Viewer | **None** — Streamer mode ≠ Go Live |
| G9 | Streamer mode missing from wiki/site docs | Cite [PR #82](https://github.com/ape-squared/Block-Viewer/pull/82) + `en.ftl` until public docs exist |

---

## Screenshot & illustration requirements

Editors should capture these (mark placeholders `![SHOT: …]` in drafts):

### OBS

1. Auto-Configuration Wizard summary screen  
2. Scenes list with Starting Soon / Live / BRB / Ending  
3. Sources list for a Live scene (capture + webcam + mic)  
4. Audio Mixer with healthy mic (−12 to −6 dB peaks)  
5. Filters list on mic (noise suppression → compressor → limiter)  
6. Settings → Output → Streaming (encoder, bitrate)  
7. Settings → Video (canvas vs output resolution)  
8. Black screen failure example + fixed Window Capture (side-by-side)  

### Kick

9. Creator Dashboard overview (blur any private IDs)  
10. Stream key page with key **fully redacted**  
11. Title / category / tags fields before Go Live  
12. Live indicator + chat panel  

### Aitum

13. Multistream destination list (Kick + one other, keys redacted)  
14. Encoder / performance note UI if present  

### Block Viewer

15. Download page on blockviewer.org  
16. Unzipped folder → launching the viewer  
17. Login screen  
18. In-world view suitable for stream (clean UI)  
18b. **Streamer mode** — Preferences › Layout toggle; Me menu; toolbar button; before/after chrome  
18c. L$ balance visible vs hidden  
19. Poser UI (for photo/content chapter)  
20. Discord widget / community CTA on site (optional)  
21. Branding swatches from branding doc (optional)  

### Content / graphics

22. Example Starting Soon / BRB / Ending full-bleed layouts (non-cluttered)  
23. Webcam frame example (simple, readable)  
24. Clip → short vertical export timeline (any editor)  

### Safety

25. “Never share stream key” callout graphic  

---

## Version freeze checklist (before class materials ship)

- [ ] OBS Studio version used in class screenshots  
- [ ] Kick Help Center article URLs still resolve  
- [ ] Aitum Multistream version + supported destinations  
- [ ] Block Viewer release tag / zip name matches download page  
- [ ] Discord invite / soft-open wording still accurate  
- [ ] Re-read `docs/BUG_REPORTING.md` overlay hooks list  
- [ ] Confirm with maintainers: Game Capture vs Window Capture guidance (G1/G5)  
- [ ] Check whether Streamer mode gained a wiki/site page (G9); update citations  
- [ ] Confirm class build includes PR #82 Streamer mode  
