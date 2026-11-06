<p align="center">
  <img src="assets/branding/block/01-logos/lockup/block-lockup-on-dark-240.png" alt="Block" width="280">
</p>

# Block Viewer

**Build your look, block by block.**

A Windows Second Life® client with a Vulkan renderer. Source and releases are public under **GPL-3.0-or-later**. Early **0.x** — fine for friends, engineers, and curious testers; not a polished product pitch.

| | |
|---|---|
| **Site** | https://ape-squared.github.io/Block-Viewer/ |
| **Releases** | https://github.com/ape-squared/Block-Viewer/releases |
| **Feedback** | https://feedback.ape-squared.github.io/Block |
| **Discord** | Release notes land in `#viewer-releases` |

---

## About this project

Block Viewer is a **hobby fork** for friends and engineers who like open source. We ship builds when something useful lands; we are **not** trying to replace or compete with other viewers.

### Built on Block Viewer

This codebase starts from the open-source **[Block Viewer](https://github.com/Aurora-Viewer/Aurora-Viewer)** project (engine, protocol work, GPL lineage). We are grateful for that work.

- Block is a **separate product and brand** — skins, packaging, and direction of our own.
- It is **not** an official Aurora release and is **not** affiliated with the Block Viewer team.
- We keep source and releases public because that is how GPL and good open-source practice work, and because Aurora’s openness is what made a fork like this possible.

Licence text and third-party credits: [LICENSE.md](LICENSE.md), [LICENSE](LICENSE), [NOTICE.md](NOTICE.md).

Not affiliated with Linden Lab. Second Life® is a trademark of Linden Research, Inc.

Sibling (different repo/licence): the [Block](https://github.com/ape-squared/Block) post-processing injector.

---

## What’s in the box

- Vulkan world rendering
- Login, destinations, MFA
- Nearby chat, IMs, groups, voice
- People / radar (including chat-range notices)
- Inventory, appearance, AO
- Environment tools (personal lighting, Edit Sky / Edit Water)
- Auto-update from GitHub Releases (zip + `.sha256`)
- Portable Windows zip with `Uninstall.cmd`

---

## Download (Windows)

1. Grab the latest [release](https://github.com/ape-squared/Block-Viewer/releases/latest).
2. Unpack `Block-Viewer-vX.Y.Z-windows-x64.zip` and run `block-viewer.exe`.
3. Optional: `Uninstall.cmd` in that folder.

Config / cache: `%LOCALAPPDATA%\ape-squared\BlockViewer\` and `%APPDATA%\ape-squared\BlockViewer\`

---

## Build from source

Rust stable (`rust-toolchain.toml`), Windows MSVC, Vulkan-capable GPU.

```powershell
powershell -ExecutionPolicy Bypass -File scripts\build-block-prod.ps1
# → dist\Block-Viewer\

cargo build -p block-viewer            # debug
cargo build -p block-viewer --release  # release
```

Dev helpers: `block-tools.cmd` (Block Tools). Old `aurora-tools.cmd` still launches it.

---

## Guides

These live in `docs/` and publish to the
[wiki](https://github.com/ape-squared/Block-Viewer/wiki).

**Testers**

- [Reporting bugs](docs/BUG_REPORTING.md)
- [Installing on Linux](docs/LINUX.md)
- [Installing on macOS](docs/MACOS.md)
- [Updates](docs/UPDATES.md)
- [Asset imports](docs/IMPORTS.md)
- [LiveMesh](docs/BLENDER_LIVE.md)
- [Motorcycle hand contacts](docs/HAND_CONTACTS.md)
- [IK Edit and bone contacts](docs/IK_EDIT.md)
- [Create native animations](docs/ANIMATION_CREATION.md)
- [Local photo poser](docs/POSER.md)

**Contributing**

- [Translating the viewer](docs/TRANSLATING.md)
- [Branding and interface](docs/BRANDING.md)

**Technical**

- [RLVa implementation status](docs/RLVA.md)
- [Licences and notices](NOTICE.md)

---

## Brand

| Token | Hex |
|-------|-----|
| signal | `#D4FF3F` |
| data | `#5BE3E8` |
| surface-0 | `#0B0C0E` |
| ink | `#ECEEF0` |

More in [docs/BRANDING.md](docs/BRANDING.md).
