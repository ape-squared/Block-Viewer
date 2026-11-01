<p align="center">
  <img src="assets/branding/block/01-logos/lockup/block-lockup-on-dark-240.png" alt="Block" width="280">
</p>

# Block Viewer

**Build your look, block by block.**  
A modern Second Life client for Windows — Vulkan renderer, Block branding, open source (GPLv3).

> Not affiliated with Linden Lab. Not affiliated with the Aurora Viewer project as a product — this tree is a **rebranded derivative** of their open-source client; see [NOTICE.md](NOTICE.md).

Sibling to the [Block](https://github.com/ape-squared/Block) post-processing injector (`C:\Users\Shadow\block`). Keep the two repos separate (different licenses and jobs).

## Status

Early 0.x. Usable for explore / chat / testing. Prefer honest bug reports over marketing claims.

## Brand

| Token | Hex |
|-------|-----|
| signal | `#D4FF3F` |
| data | `#5BE3E8` |
| surface-0 | `#0B0C0E` |
| ink | `#ECEEF0` |

Login uses a modular grid wash over dark surfaces; accent roles in code still named `violet`/`indigo` map to signal/data.

## Build

Rust stable (see `rust-toolchain.toml`). Windows MSVC + Vulkan-capable GPU.

```powershell
# Demo — offline plaza, fast local profile
powershell -ExecutionPolicy Bypass -File scripts\build-block-demo.ps1

# Production — optimized Release portable folder
powershell -ExecutionPolicy Bypass -File scripts\build-block-prod.ps1
```

| Artifact | Path | Notes |
|----------|------|--------|
| Demo | `dist\Block-Viewer-Demo\BlockViewer.exe` | Offline plaza via `BlockViewer-Demo.bat` (`AURORA_DEMO=1`) |
| Production | `dist\Block-Viewer\BlockViewer.exe` | Optimized Release, main-grid ready |

Raw cargo:

```powershell
cargo build -p block-viewer            # Dev profile → target\debug\
cargo build -p block-viewer --release  # Production → target\release\
```

Internal crate names still use the `block-*` prefix (Rust graph). User-visible product name is **Block Viewer**.

## Config location

Windows: `%LOCALAPPDATA%\ape-squared\BlockViewer\`

## License

GPL-3.0-or-later — [LICENSE](LICENSE). Upstream credits — [NOTICE.md](NOTICE.md).
