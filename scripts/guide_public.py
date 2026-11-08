#!/usr/bin/env python3
"""Trim long guides for the published site and wiki.

Full Markdown in docs/ stays the source of truth for contributors. Published
pages drop protocol, offline-validation, and other deep sections so GitHub
wiki and the site stay readable.
"""

from __future__ import annotations

import argparse
from pathlib import Path

# Cut from this heading (exact line match after strip) through end of file.
PUBLIC_CUT_FROM: dict[str, str] = {
    "docs/IK_EDIT.md": "## Frames and protocol",
    # Drop rig math + offline/wire dumps; Current limits is re-appended below.
    "docs/BLENDER_LIVE.md": "## Rig and coordinate conventions",
    "docs/RLVA.md": "## Offline validation",
    "docs/IMPORTS.md": "## Validation",
    "docs/BUG_REPORTING.md": "### Validation",
    "docs/BRANDING.md": "## Icons: Tabler",
}

# After cutting, re-attach these H2 sections from the original (in order).
PUBLIC_KEEP_AFTER_CUT: dict[str, list[str]] = {
    "docs/BLENDER_LIVE.md": ["## Current limits"],
}

OMIT_NOTE = (
    "\n\n---\n\n"
    "*Deeper protocol, offline-validation, and contributor-depth sections "
    "are omitted here so this page stays readable.*\n"
)


def _normalize(source: str) -> str:
    return source.replace("\\", "/")


def _heading_level(line: str) -> int | None:
    s = line.strip()
    if not s.startswith("#"):
        return None
    n = 0
    for ch in s:
        if ch == "#":
            n += 1
        else:
            break
    if n == 0 or n > 6 or len(s) == n or s[n] != " ":
        return None
    return n


def _extract_section(text: str, heading: str) -> str | None:
    """Return one markdown section from heading through the next same-level heading."""
    lines = text.replace("\r\n", "\n").split("\n")
    level = _heading_level(heading)
    if level is None:
        return None
    start = None
    for i, line in enumerate(lines):
        if line.strip() == heading:
            start = i
            break
    if start is None:
        return None
    end = len(lines)
    for j in range(start + 1, len(lines)):
        other = _heading_level(lines[j])
        if other is not None and other <= level:
            end = j
            break
    return "\n".join(lines[start:end]).rstrip()


def public_markdown(source: str, text: str) -> str:
    """Return the published (possibly shortened) form of a guide."""
    source = _normalize(source)
    cut = PUBLIC_CUT_FROM.get(source)
    if not cut:
        return text

    text = text.replace("\r\n", "\n")
    kept_extra: list[str] = []
    for heading in PUBLIC_KEEP_AFTER_CUT.get(source, []):
        section = _extract_section(text, heading)
        if section:
            kept_extra.append(section)

    lines = text.split("\n")
    for i, line in enumerate(lines):
        if line.strip() == cut:
            kept = "\n".join(lines[:i]).rstrip()
            if kept_extra:
                kept = kept + "\n\n" + "\n\n".join(kept_extra)
            return kept.rstrip() + OMIT_NOTE
    return text


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--source", required=True, help="Repo-relative guide path")
    ap.add_argument(
        "--repo",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="Repository root",
    )
    ap.add_argument("--write", type=Path, required=True, help="Output markdown path")
    args = ap.parse_args()
    raw = (args.repo / args.source).read_text(encoding="utf-8")
    args.write.parent.mkdir(parents=True, exist_ok=True)
    args.write.write_text(public_markdown(args.source, raw), encoding="utf-8")


if __name__ == "__main__":
    main()
