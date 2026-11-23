#!/usr/bin/env python3
"""Build and POST a Block Viewer release embed to Discord (GitHub Actions)."""

import json
import os
import re
import sys
import urllib.request


def release_url(release: dict) -> str:
    return release.get("url") or release.get("html_url") or release.get("htmlUrl") or ""


def is_prerelease(release: dict) -> bool:
    if "isPrerelease" in release:
        return bool(release["isPrerelease"])
    return bool(release.get("prerelease"))


def asset_download(asset: dict) -> str:
    return asset.get("url") or asset.get("browser_download_url") or asset.get("browserDownloadUrl") or ""


def main() -> None:
    release = json.loads(os.environ["RELEASE_JSON"])
    webhook = os.environ["WEBHOOK"].rstrip("/")
    if webhook.endswith("/github"):
        webhook = webhook[: -len("/github")]

    tag = release.get("tagName") or release.get("tag_name") or ""
    name = (release.get("name") or tag or "Release").strip()
    url = release_url(release)
    prerelease = is_prerelease(release)
    body = (release.get("body") or "").strip()
    body = re.sub(r"<!--.*?-->", "", body, flags=re.S)
    body = re.sub(r"(?im)^#+\s*Block Viewer\s*$", "", body)
    body = re.sub(r"(?im)^Support:.*$", "", body)
    body = re.sub(r"(?im)^Sister project.*$", "", body)
    body = re.sub(r"\n{3,}", "\n\n", body).strip()
    if len(body) > 1800:
        body = body[:1797].rstrip() + "…"
    if not body:
        body = "Release notes are on GitHub."

    assets = release.get("assets") or []
    windows = next(
        (
            a
            for a in assets
            if a.get("name", "").endswith("-windows-x64.zip") and not a.get("name", "").endswith("-symbols.zip")
        ),
        None,
    )
    fields = []
    if windows and asset_download(windows):
        fields.append({"name": "Windows", "value": f"[Download zip]({asset_download(windows)})", "inline": True})
    fields.append({"name": "Tag", "value": f"[`{tag}`]({url})" if tag and url else (tag or "—"), "inline": True})
    color = 0x38BDF8 if prerelease else 0xD4FF3F
    title = name if name.lower().startswith("block") else f"Block Viewer {name}"
    thumb = "https://raw.githubusercontent.com/ape-squared/Block-Viewer/main/assets/branding/block-logo-bg.png"
    embed = {
        "title": title,
        "url": url,
        "description": body,
        "color": color,
        "fields": fields,
        "thumbnail": {"url": thumb},
        "footer": {"text": "Block Viewer · GitHub Releases"},
    }
    published = release.get("publishedAt") or release.get("published_at")
    if published:
        embed["timestamp"] = published

    kind = "New pre-release" if prerelease else "New release"
    role = (os.environ.get("TESTER_ROLE") or "").strip()
    allowed = {"parse": []}
    if role.isdigit():
        content = f"{kind} <@&{role}>"
        allowed = {"roles": [role]}
    else:
        content = f"{kind} — testers welcome"
    payload = {
        "username": "Block Viewer",
        "avatar_url": thumb,
        "content": content,
        "embeds": [embed],
        "allowed_mentions": allowed,
    }
    req = urllib.request.Request(
        webhook + "?wait=true",
        data=json.dumps(payload).encode(),
        headers={"Content-Type": "application/json", "User-Agent": "Block-Viewer-Release-Notify"},
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        print(f"Discord {resp.status}")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"discord embed failed: {e}", file=sys.stderr)
        raise