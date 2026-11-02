<p align="center">
  <img src="assets/branding/block/01-logos/lockup/block-lockup-on-dark-240.png" alt="Block" width="280">
</p>

# Block Viewer

**Build your look, block by block.**

A modern Second Life® client for Windows — Vulkan renderer, open source under **GPL-3.0-or-later**.

Block Viewer is built on the open-source [Aurora Viewer](https://github.com/Aurora-Viewer/Aurora-Viewer) codebase (engine, protocol, GPL lineage — see [NOTICE.md](NOTICE.md)). The product identity, UI, and direction are **Block**: not an official Aurora release, and not affiliated with Linden Lab.

Sibling project: the [Block](https://github.com/ape-squared/Block) post-processing injector ([site](https://ape-squared.github.io/Block-GPPI/)). Keep the repos separate (different licenses and jobs).

| | |
|---|---|
| **Site** | https://ape-squared.github.io/Block-Viewer/ |
| **Releases** | https://github.com/ape-squared/Block-Viewer/releases |
| **Feedback** | https://feedback.ape-squared.github.io/Block |
| **Discord** | Release posts go to `#viewer-releases` |

---

## What’s in the box

- Vulkan world renderer (smooth frame pacing, GPU-friendly uploads)
- Login with destinations, cached favourite landmarks, MFA support
- Nearby chat, IMs, groups, voice (incl. optional voice morph)
- People / radar with chat-range enter & leave notices (Preferences › Chat)
- Inventory, appearance, AO (`#Block` / Block AO; former Aurora folders still show as Block)
- Auto-update from GitHub Releases (zip + `.sha256`)
- Discord Rich Presence activity name **Block Viewer**
- Immersion mode (Legacy Toggle UI), build XYZ, LiveMesh branding
- Offline **demo plaza** for local testing without a grid login
- Windows portable zip with `Uninstall.cmd`
- Environment window: Personal lighting plus Edit Sky / Edit Water (Sun & Moon, atmosphere, clouds)

Early **0.x** — usable for explore / chat / testing. Prefer clear bug reports over marketing claims.

---

## Download & install (Windows)

1. Open the [latest release](https://github.com/ape-squared/Block-Viewer/releases/latest).
2. Download `Block-Viewer-vX.Y.Z-windows-x64.zip` (and the `.sha256` sidecar if you use in-viewer updates).
3. Extract anywhere and run `block-viewer.exe`.
4. To remove: double-click `Uninstall.cmd` in that folder (optional AppData wipe).

In-viewer **Check for updates** installs a verified portable package when the feed and checksum match.

**Config / cache (Windows):** `%LOCALAPPDATA%\ape-squared\BlockViewer\` and `%APPDATA%\ape-squared\BlockViewer\`

---

## Build from source

Rust stable (`rust-toolchain.toml`). Windows MSVC + a Vulkan-capable GPU.

```powershell
# Demo — offline plaza, fast local profile
powershell -ExecutionPolicy Bypass -File scripts\build-block-demo.ps1

# Production — optimized Release portable folder
powershell -ExecutionPolicy Bypass -File scripts\build-block-prod.ps1
```

| Artifact | Path | Notes |
|----------|------|--------|
| Demo | `dist\Block-Viewer-Demo\` | Offline plaza (`AURORA_DEMO=1`) |
| Production | `dist\Block-Viewer\` | Optimized Release |

Raw cargo:

```powershell
cargo build -p block-viewer            # Dev → target\debug\
cargo build -p block-viewer --release  # Production → target\release\
```

Agents and contributors: start with [AGENTS.md](AGENTS.md), [ARCHITECTURE.md](ARCHITECTURE.md), and [TASKS.md](TASKS.md). Humans: [HUMANS.md](HUMANS.md).

Internal crate names use the `block-*` prefix. User-visible product name is **Block Viewer**.

---

## Demo mode

```powershell
$env:AURORA_DEMO = "1"
cargo run -p block-viewer -- --title "Test local"
```

See [docs/TESTING.md](docs/TESTING.md) for `AURORA_*` variables, captures, and automation.

---

## Brand

| Token | Hex |
|-------|-----|
| signal | `#D4FF3F` |
| data | `#5BE3E8` |
| surface-0 | `#0B0C0E` |
| ink | `#ECEEF0` |

Login uses a modular grid wash over dark surfaces. Interface chrome follows [docs/BRANDING.md](docs/BRANDING.md).

---

## Support & team

Block developers (Heard, 4krazy) get a team nameplate and stay pinned at the top of your **Friends** list when they are friends — handy for contact and support.

---

## License

GPL-3.0-or-later — [LICENSE](LICENSE). Upstream credits — [NOTICE.md](NOTICE.md).

Second Life® is a trademark of Linden Research, Inc. This project is not affiliated with or endorsed by Linden Lab.
