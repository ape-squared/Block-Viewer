<p align="center">
  <img src="assets/branding/block/01-logos/lockup/block-lockup-on-dark-240.png" alt="Block" width="280">
</p>

# Block Viewer

**Build your look, block by block.**

A Windows Second Life® client with a Vulkan renderer. Source and releases are public under **GPL-3.0-or-later**. Early **0.x** — fine for friends, engineers, and curious testers; not a polished product pitch.

| | |
|---|---|
| **Site** | https://blockviewer.org/ |
| **Releases** | https://github.com/ape-squared/Block-Viewer/releases |
| **Feedback** | https://feedback.ape-squared.github.io/Block |
| **Discord** | Release notes land in `#viewer-releases` |

---

## About this project

Block Viewer is a **hobby fork** for friends and engineers who like open source. We ship builds when something useful lands; we are **not** trying to replace or compete with other viewers.

### Built on Aurora Viewer

Portions of this codebase are derived from the open-source **[Aurora Viewer](https://github.com/Aurora-Viewer/Aurora-Viewer)** project (engine, protocol work, GPL lineage). We are grateful for that work and keep the required upstream notices.

- Block is a **separate product and brand** — skins, packaging, modifications, and direction of our own — **not** Aurora Viewer renamed.
- It is **not** an official Aurora release and is **not** affiliated with the Aurora Viewer team.
- Block is built by **many contributors** (maintainers, testers, and engineers in this repository’s history), just as Aurora’s lineage reflects many Aurora contributors. See [NOTICE.md](NOTICE.md).
- We keep source and releases public because that is how GPL and good open-source practice work, and because Aurora’s openness is what made this possible.

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

Readable on the site: [Docs](https://blockviewer.org/docs/).
Markdown sources stay in `docs/` (and refresh the site + wiki on push).

**Testers**

- [Reporting bugs](https://blockviewer.org/docs/reporting-bugs/)
- [Installing on Linux](https://blockviewer.org/docs/linux/)
- [Experimental VR](https://blockviewer.org/docs/vr/)
- [Experimental Windows VR](https://blockviewer.org/docs/windows-vr/)
- [Installing on macOS](https://blockviewer.org/docs/macos/)
- [Updates](https://blockviewer.org/docs/updates/)
- [Asset imports](https://blockviewer.org/docs/imports/)
- [LiveMesh](https://blockviewer.org/docs/livemesh/)
- [Motorcycle hand contacts](https://blockviewer.org/docs/hand-contacts/)
- [IK Edit and bone contacts](https://blockviewer.org/docs/ik-edit/)
- [Create native animations](https://blockviewer.org/docs/animation-creation/)
- [Local photo poser](https://blockviewer.org/docs/poser/)
- [Dev Toolbox](https://blockviewer.org/docs/dev-toolbox/)

**Contributing**

- [Translating the viewer](https://blockviewer.org/docs/translating/)
- [Branding and interface](https://blockviewer.org/docs/branding/)

**Technical**

- [RLVa implementation status](https://blockviewer.org/docs/rlva/)
- [Licences and notices](https://blockviewer.org/docs/licences/)

---

## Brand

| Token | Hex |
|-------|-----|
| signal | `#D4FF3F` |
| data | `#5BE3E8` |
| surface-0 | `#0B0C0E` |
| ink | `#ECEEF0` |

More in [docs/BRANDING.md](docs/BRANDING.md).
