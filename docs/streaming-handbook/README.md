# Streaming & content creation handbook — research package

**Purpose:** Source material for Claude (or a human editor) to produce a
beginner-friendly handbook for Block Viewer free streaming / content-creation
classes. This folder is research and structure, not the final polished
handbook prose.

**Audience of the final handbook:** Absolute beginners — zero streaming
experience — learning OBS, Kick, Aitum Multistream, Block Viewer, clipping,
and basic production.

**Tone target for the final book:** Casual, practical, friendly. Explain
WHY. Numbered steps. Not corporate, not guru, not fake-advanced.

---

## How to use this package

1. Start with [`00-meta-sources-gaps.md`](00-meta-sources-gaps.md) for
   citations, gaps, and screenshot needs.
2. Write chapters from `01`–`08` in order (or parallelize after reading meta).
3. Pull FAQs / glossary / support framing from `09`–`11`.
4. Prefer **verified** bullets (linked) over **practical advice** (labeled).
5. Never invent Block Viewer features. If a wiki page is silent, leave a
   **[NEEDS CONFIRMATION]** note.

---

## Table of contents (final handbook)

1. Welcome & how to use this handbook (with live class / Discord support)
2. Quick path: first livestream checklist
3. OBS Studio — complete beginner setup → [`01-obs-studio.md`](01-obs-studio.md)
4. Kick — account, dashboard, go live → [`02-kick.md`](02-kick.md)
5. Aitum Multistream → [`03-aitum-multistream.md`](03-aitum-multistream.md)
6. **Block Viewer for creators** → [`04-block-viewer.md`](04-block-viewer.md)
7. Turning streams into content → [`05-content-creation.md`](05-content-creation.md)
8. Stream graphics & branding → [`06-graphics-branding.md`](06-graphics-branding.md)
9. Audio, lighting & budget gear → [`07-audio-lighting-gear.md`](07-audio-lighting-gear.md)
10. Troubleshooting & optimization → [`08-troubleshooting.md`](08-troubleshooting.md)
11. Learning with the community → [`09-support-structure.md`](09-support-structure.md)
12. Glossary → [`10-glossary.md`](10-glossary.md)
13. FAQ → [`11-faqs.md`](11-faqs.md)

---

## Block Viewer documentation map (primary product sources)

| Topic | Site / wiki | Repo source |
| --- | --- | --- |
| Product overview / download | [Home wiki](https://github.com/ape-squared/Block-Viewer/wiki/Home), [blockviewer.org](https://blockviewer.org/) | `README.md` |
| Streamer mode (client; wiki pending) | [PR #82](https://github.com/ape-squared/Block-Viewer/pull/82) | `en.ftl` + `app.rs` — see `04-block-viewer.md` §E0 |
| Windows download notes | [Download](https://blockviewer.org/download/) | `website/download/` |
| Updates | [Updates](https://blockviewer.org/docs/updates/), [wiki Updates](https://github.com/ape-squared/Block-Viewer/wiki/Updates) | `docs/UPDATES.md` |
| Reporting bugs / overlays | [Reporting bugs](https://blockviewer.org/docs/reporting-bugs/) | `docs/BUG_REPORTING.md` |
| Local photo poser | [Poser](https://blockviewer.org/docs/poser/) | `docs/POSER.md` |
| IK / animation (advanced creator) | [IK Edit](https://blockviewer.org/docs/ik-edit/), [Animation creation](https://blockviewer.org/docs/animation-creation/) | `docs/IK_EDIT.md`, `docs/ANIMATION_CREATION.md` |
| LiveMesh | [LiveMesh](https://blockviewer.org/docs/livemesh/) | `docs/BLENDER_LIVE.md` |
| Branding / palette | [Branding](https://blockviewer.org/docs/branding/) | `docs/BRANDING.md` |
| Dev Toolbox (diagnostics) | [Dev Toolbox](https://blockviewer.org/docs/dev-toolbox/) | `docs/DEV_TOOLBOX.md` |
| Tester nameplate signup | [Testers](https://blockviewer.org/testers/) | `peers/block-staff.json` + site |
| Discord (soft open) | Home `#discord` | `website/index.html` |
| Linux / macOS install | [Linux](https://blockviewer.org/docs/linux/), [macOS](https://blockviewer.org/docs/macos/) | `docs/LINUX.md`, `docs/MACOS.md` |

Wiki pages are generated from the same Markdown as the site
(`scripts/build-wiki.py` / `build-site-docs.py`). Prefer **site URLs** for
readers; cite **wiki** when the class materials already link there.

---

## Status

| File | Depth |
| --- | --- |
| Meta / gaps / screenshots | Ready for editors |
| OBS / Kick / Aitum | Ready (official docs cited; re-check version dates before publish) |
| Block Viewer | Ready from current wiki/docs; capture path marked for confirmation |
| Content / graphics / gear | Practical guidance + branded Block notes |
| Troubleshooting / FAQ / glossary / support | Ready |

**Last researched against repo `main` around 2026-11-14.** Software UIs
(Kick, OBS, Aitum) change — re-verify linked official pages before class
print/PDF freeze.
