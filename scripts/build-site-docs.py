#!/usr/bin/env python3
"""Generate website/docs HTML pages from the same Markdown guides as the wiki.

The site hosts the guides on GitHub Pages so visitors are not sent to GitHub
blobs or the wiki for everyday reading. Edit the .md sources in the repo;
run this script (Pages CI does too) to refresh website/docs/.
"""

from __future__ import annotations

import argparse
import html
import os
import re
import shutil
import sys
from pathlib import Path

try:
    import markdown
except ImportError:
    sys.stderr.write("Install the markdown package: pip install markdown\n")
    raise SystemExit(1)

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from guide_public import public_markdown  # noqa: E402

OUT = ROOT / "website" / "docs"

# Keep in sync with scripts/build-wiki.ps1 page table (site slugs are lowercase).
PAGES: list[dict] = [
    {"section": None, "slug": "", "title": "Docs", "source": None},  # hub only
    {"section": "For testers", "slug": "reporting-bugs", "title": "Reporting bugs", "source": "docs/BUG_REPORTING.md"},
    {"section": "For testers", "slug": "linux", "title": "Installing on Linux", "source": "docs/LINUX.md"},
    {"section": "For testers", "slug": "vr", "title": "Experimental VR scene viewing", "source": "docs/VR.md"},
    {"section": "For testers", "slug": "windows-vr", "title": "Experimental Windows VR", "source": "docs/VR_WINDOWS.md"},
    {"section": "For testers", "slug": "macos", "title": "Installing on macOS", "source": "docs/MACOS.md"},
    {"section": "For testers", "slug": "updates", "title": "Updates", "source": "docs/UPDATES.md"},
    {"section": "For testers", "slug": "imports", "title": "Asset imports", "source": "docs/IMPORTS.md"},
    {"section": "For testers", "slug": "livemesh", "title": "LiveMesh", "source": "docs/BLENDER_LIVE.md"},
    {"section": "For testers", "slug": "hand-contacts", "title": "Motorcycle hand contacts", "source": "docs/HAND_CONTACTS.md"},
    {"section": "For testers", "slug": "ik-edit", "title": "IK Edit and bone contacts", "source": "docs/IK_EDIT.md"},
    {"section": "For testers", "slug": "animation-creation", "title": "Create native animations", "source": "docs/ANIMATION_CREATION.md"},
    {"section": "For testers", "slug": "poser", "title": "Local photo poser", "source": "docs/POSER.md"},
    {"section": "For testers", "slug": "dev-toolbox", "title": "Dev Toolbox", "source": "docs/DEV_TOOLBOX.md"},
    {"section": "Contributing", "slug": "translating", "title": "Translating the viewer", "source": "docs/TRANSLATING.md"},
    {"section": "Contributing", "slug": "branding", "title": "Branding and interface", "source": "docs/BRANDING.md"},
    {"section": "Technical", "slug": "rlva", "title": "RLVa implementation status", "source": "docs/RLVA.md"},
    {"section": "Technical", "slug": "licences", "title": "Licences and notices", "source": "NOTICE.md"},
]

GUIDE_PAGES = [p for p in PAGES if p["source"]]
PAGE_OF = {p["source"]: p["slug"] for p in GUIDE_PAGES}

MD = markdown.Markdown(
    extensions=["tables", "fenced_code", "toc", "sane_lists", "smarty"],
    extension_configs={"toc": {"permalink": False}},
)


def resolve_target(from_path: str, target: str) -> str:
    parts: list[str] = []
    d = os.path.dirname(from_path)
    if d:
        parts.extend(s for s in d.replace("\\", "/").split("/") if s)
    for s in target.split("/"):
        if s == "..":
            if parts:
                parts.pop()
        elif s and s != ".":
            parts.append(s)
    return "/".join(parts)


def rewrite_md_links(source: str, text: str) -> str:
    def repl(m: re.Match[str]) -> str:
        label, target = m.group(1), m.group(2)
        if re.match(r"^([a-z][a-z0-9+.-]*:|#|//)", target):
            return m.group(0)
        path, *anchor = target.split("#", 1)
        file = resolve_target(source, path) if path else source
        suffix = f"#{anchor[0]}" if anchor else ""
        if file in PAGE_OF:
            slug = PAGE_OF[file]
            return f"[{label}](../{slug}/{suffix})" if slug else f"[{label}](../{suffix})"
        # Leave other repo files as GitHub blob links (source, licences binaries, etc.)
        if path and (ROOT / file).exists():
            url = f"https://github.com/ape-squared/Block-Viewer/blob/main/{file}{suffix}"
            return f"[{label}]({url})"
        return m.group(0)

    return re.sub(r"\[([^\]]+)\]\(([^)\s]+)\)", repl, text)


def chrome(title: str, active: str, body: str, *, hub: bool = False) -> str:
    # Hub lives at website/docs/; guides live one folder deeper.
    root = ".." if hub else "../.."
    css = f"{root}/site.css?v=14"
    js = f"{root}/app.js?v=11"
    mark = f"{root}/assets/mark.svg"
    home = f"{root}/"
    download = f"{root}/download/"
    docs = f"{root}/docs/" if not hub else "./"
    foundation = f"{root}/foundation/"
    privacy = f"{root}/privacy/"
    canonical = (
        "https://ape-squared.github.io/Block-Viewer/docs/"
        if hub
        else f"https://ape-squared.github.io/Block-Viewer/docs/{active}/"
    )
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{html.escape(title)} — Block Viewer</title>
  <meta name="description" content="{html.escape(title)} — Block Viewer documentation.">
  <link rel="canonical" href="{canonical}">
  <link rel="icon" href="{mark}" type="image/svg+xml">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@500;700&family=IBM+Plex+Sans:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="{css}">
</head>
<body>
  <a class="skip" href="#main">Skip to content</a>
  <div class="atmosphere" aria-hidden="true"></div>
  <header class="nav" data-nav>
    <a class="brand" href="{home}" aria-label="Block Viewer home">
      <img src="{mark}" width="28" height="28" alt="">
      <span>BLOCK</span>
    </a>
    <button class="nav-toggle" type="button" aria-expanded="false" aria-controls="site-nav" data-nav-toggle>
      <span class="nav-toggle-bar"></span>
      <span class="sr-only">Menu</span>
    </button>
    <nav id="site-nav" class="nav-links" aria-label="Primary">
      <a href="{download}">Download</a>
      <a href="{home}#features">Features</a>
      <a class="is-active" href="{docs}">Docs</a>
      <a href="{foundation}">Foundation</a>
    </nav>
    <a class="btn btn-primary nav-cta" href="{download}">Download</a>
  </header>
  <main id="main" class="section page-shell docs-layout">
{body}
  </main>
  <footer class="footer">
    <div class="footer-row">
      <a class="brand" href="{home}"><img src="{mark}" width="22" height="22" alt=""><span>BLOCK VIEWER</span></a>
      <nav class="footer-links" aria-label="Footer">
        <a href="{download}">Download</a>
        <a href="{docs}">Docs</a>
        <a href="{foundation}">Foundation</a>
        <a href="{privacy}">Privacy</a>
        <a href="https://github.com/ape-squared/Block-Viewer">GitHub</a>
      </nav>
    </div>
    <p class="legal">Guides are hosted on this site. Source of truth is the Markdown in the repository.</p>
  </footer>
  <script src="{js}" defer></script>
</body>
</html>
"""


def sidebar_html(active: str) -> str:
    parts = [
        '<aside class="docs-side" aria-label="Guides">',
        '<p class="docs-side-title"><a href="../">All guides</a></p>',
    ]
    section = None
    for p in GUIDE_PAGES:
        if p["section"] != section:
            section = p["section"]
            parts.append(f'<p class="docs-side-section">{html.escape(section)}</p>')
        cls = ' class="is-active"' if p["slug"] == active else ""
        parts.append(f'<a{cls} href="../{p["slug"]}/">{html.escape(p["title"])}</a>')
    parts.append("</aside>")
    return "\n".join(parts)


def hub_page() -> str:
    blocks = [
        '    <header class="page-hero reveal">',
        '      <p class="eyebrow">Guides</p>',
        "      <h1>Docs</h1>",
        "      <p class=\"lede\">Installer notes, creator tools, translating, and technical status — hosted here on the Block Viewer site.</p>",
        "    </header>",
        '    <div class="docs-hub">',
    ]
    section = None
    for p in GUIDE_PAGES:
        if p["section"] != section:
            if section is not None:
                blocks.append("      </ul>")
            section = p["section"]
            blocks.append(f'      <h2 class="docs-hub-section reveal">{html.escape(section)}</h2>')
            blocks.append('      <ul class="doc-grid">')
        blocks.append(
            f'        <li><a class="doc-link reveal" href="{p["slug"]}/">'
            f"<strong>{html.escape(p['title'])}</strong>"
            f"<span>Open guide</span></a></li>"
        )
    blocks.append("      </ul>")
    blocks.append("    </div>")
    return chrome("Docs", "", "\n".join(blocks), hub=True)


def guide_page(page: dict) -> str:
    source = page["source"]
    raw = public_markdown(source, (ROOT / source).read_text(encoding="utf-8"))
    raw = rewrite_md_links(source, raw)
    MD.reset()
    body_html = MD.convert(raw)
    # Drop duplicate H1 if present — page hero carries the title
    body_html = re.sub(r"^<h1[^>]*>.*?</h1>\s*", "", body_html, count=1, flags=re.I | re.S)
    content = f"""    <div class="docs-with-side">
      {sidebar_html(page["slug"])}
      <article class="prose docs-article">
        <p class="eyebrow">{html.escape(page["section"] or "Guide")}</p>
        <h1>{html.escape(page["title"])}</h1>
        {body_html}
      </article>
    </div>"""
    return chrome(page["title"], page["slug"], content)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", type=Path, default=OUT)
    args = ap.parse_args()
    out: Path = args.out
    if out.exists():
        for child in out.iterdir():
            if child.name == ".gitkeep":
                continue
            if child.is_dir():
                shutil.rmtree(child)
            elif child.suffix == ".html":
                child.unlink()
    out.mkdir(parents=True, exist_ok=True)

    (out / "index.html").write_text(hub_page(), encoding="utf-8")
    for page in GUIDE_PAGES:
        if not (ROOT / page["source"]).exists():
            raise SystemExit(f"missing source {page['source']}")
        dest = out / page["slug"]
        dest.mkdir(parents=True, exist_ok=True)
        (dest / "index.html").write_text(guide_page(page), encoding="utf-8")
        print(f"  {page['slug']}/")
    print(f"{len(GUIDE_PAGES)} guides + hub → {out}")


if __name__ == "__main__":
    main()
