# Block Viewer

Sibling product to the Block injector (`C:\Users\Shadow\block`).

- **Product name:** Block Viewer  
- **License:** GPL-3.0-or-later (derivative of the open-source Aurora Viewer codebase — credit in [NOTICE.md](NOTICE.md); do not ship Aurora as the product name)  
- **Brand:** signal `#D4FF3F`, data `#5BE3E8`, surfaces from `#0B0C0E` — see [docs/BRANDING.md](docs/BRANDING.md)

## Packages

```powershell
powershell -ExecutionPolicy Bypass -File scripts\build-block-demo.ps1
powershell -ExecutionPolicy Bypass -File scripts\build-block-prod.ps1
```

| Build | Folder | Binary |
|-------|--------|--------|
| Demo (offline plaza) | `dist\Block-Viewer-Demo\` | `BlockViewer.exe` + `BlockViewer-Demo.bat` |
| Production (Release) | `dist\Block-Viewer\` | `BlockViewer.exe` + `BlockViewer.bat` |

Internal Cargo crate names still use the `block-*` prefix; that is build graph only.
