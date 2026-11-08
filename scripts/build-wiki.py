#!/usr/bin/env python3
"""Generate GitHub wiki pages from repository guides (Python twin of build-wiki.ps1)."""

from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from guide_public import public_markdown  # noqa: E402

PAGES: list[dict] = [
    {"section": None, "page": "Home", "title": "Home", "source": "README.md"},
    {"section": "For testers", "page": "Reporting-bugs", "title": "Reporting bugs", "source": "docs/BUG_REPORTING.md"},
    {"section": "For testers", "page": "Installing-on-Linux", "title": "Installing on Linux", "source": "docs/LINUX.md"},
    {"section": "For testers", "page": "VR", "title": "Experimental VR scene viewing", "source": "docs/VR.md"},
    {"section": "For testers", "page": "Windows-VR", "title": "Experimental Windows VR", "source": "docs/VR_WINDOWS.md"},
    {"section": "For testers", "page": "Installing-on-macOS", "title": "Installing on macOS", "source": "docs/MACOS.md"},
    {"section": "For testers", "page": "Updates", "title": "Updates", "source": "docs/UPDATES.md"},
    {"section": "For testers", "page": "Asset-imports", "title": "Asset imports", "source": "docs/IMPORTS.md"},
    {"section": "For testers", "page": "LiveMesh", "title": "LiveMesh", "source": "docs/BLENDER_LIVE.md"},
    {"section": "For testers", "page": "Hand-contacts", "title": "Motorcycle hand contacts", "source": "docs/HAND_CONTACTS.md"},
    {"section": "For testers", "page": "IK-Edit", "title": "IK Edit and bone contacts", "source": "docs/IK_EDIT.md"},
    {"section": "For testers", "page": "Animation-creation", "title": "Create native animations", "source": "docs/ANIMATION_CREATION.md"},
    {"section": "For testers", "page": "Poser", "title": "Local photo poser", "source": "docs/POSER.md"},
    {"section": "For testers", "page": "Dev-Toolbox", "title": "Dev Toolbox", "source": "docs/DEV_TOOLBOX.md"},
    {"section": "Contributing", "page": "Translating", "title": "Translating the viewer", "source": "docs/TRANSLATING.md"},
    {"section": "Contributing", "page": "Branding", "title": "Branding and interface", "source": "docs/BRANDING.md"},
    {"section": "Technical", "page": "RLVa", "title": "RLVa implementation status", "source": "docs/RLVA.md"},
    {"section": "Technical", "page": "Licences", "title": "Licences and notices", "source": "NOTICE.md"},
]

PAGE_OF = {p["source"]: p["page"] for p in PAGES}


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


def convert_link(
    source: str,
    target: str,
    image: bool,
    *,
    repo: str,
    branch: str,
    root: Path,
) -> str:
    if re.match(r"^([a-z][a-z0-9+.-]*:|#|//)", target):
        return target
    path, *anchor = target.split("#", 1)
    file = resolve_target(source, path) if path else source
    suffix = f"#{anchor[0]}" if anchor else ""
    if not image and file in PAGE_OF:
        return PAGE_OF[file] + suffix
    full = root / file
    if not full.exists():
        raise SystemExit(f"{source} links to '{target}', which is not in the repository.")
    blob = f"https://github.com/{repo}/blob/{branch}"
    raw = f"https://raw.githubusercontent.com/{repo}/{branch}"
    if image or re.search(r"\.(png|jpe?g|gif|svg|webp)$", file, re.I):
        return f"{raw}/{file}"
    if full.is_dir():
        return f"https://github.com/{repo}/tree/{branch}/{file}"
    return f"{blob}/{file}{suffix}"


def rewrite_line(source: str, line: str, *, repo: str, branch: str, root: Path) -> str:
    def link_repl(m: re.Match[str]) -> str:
        return "](" + convert_link(source, m.group(1), False, repo=repo, branch=branch, root=root) + ")"

    def attr_repl(m: re.Match[str]) -> str:
        kind, target = m.group(1), m.group(2)
        return f'{kind}="{convert_link(source, target, kind == "src", repo=repo, branch=branch, root=root)}"'

    line = re.sub(r"\]\(([^)\s]+)\)", link_repl, line)
    line = re.sub(r'\b(src|href)="([^"]+)"', attr_repl, line)
    return line


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--repository", default=os.environ.get("GITHUB_REPOSITORY", "ape-squared/Block-Viewer"))
    ap.add_argument("--branch", default="main")
    args = ap.parse_args()
    out: Path = args.out
    out.mkdir(parents=True, exist_ok=True)
    for old in out.glob("*.md"):
        old.unlink()

    blob = f"https://github.com/{args.repository}/blob/{args.branch}"
    for p in PAGES:
        source = p["source"]
        raw = public_markdown(source, (ROOT / source).read_text(encoding="utf-8"))
        lines = raw.replace("\r\n", "\n").split("\n")
        fenced = False
        body: list[str] = []
        for line in lines:
            if re.match(r"^\s*```", line):
                fenced = not fenced
                body.append(line)
                continue
            if fenced:
                body.append(line)
                continue
            body.append(
                rewrite_line(source, line, repo=args.repository, branch=args.branch, root=ROOT)
            )
        note = (
            f"> This page is generated from [{source}]({blob}/{source}). "
            "To change it, edit that file in the repository: edits made here are replaced."
        )
        (out / f"{p['page']}.md").write_text(note + "\n\n" + "\n".join(body), encoding="utf-8")
        print(f"  {p['page']}.md")

    sidebar: list[str] = []
    section = ""
    for p in PAGES:
        if p["section"] and p["section"] != section:
            section = p["section"]
            sidebar.extend(["", f"**{section}**", ""])
        sidebar.append(f"- [{p['title']}]({p['page']})")
    sidebar.extend(["", "**In the repository**", ""])
    (out / "_Sidebar.md").write_text("\n".join(sidebar) + "\n", encoding="utf-8")
    (out / "_Footer.md").write_text(
        f"Generated from the [repository](https://github.com/{args.repository}); the guides are edited there.\n",
        encoding="utf-8",
    )
    print(f"{len(PAGES)} pages written to {out}")


if __name__ == "__main__":
    main()
