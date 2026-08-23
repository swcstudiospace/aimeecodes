#!/usr/bin/env python3
"""Verify a GitBook docs space inside a product repo.

Checks (all must pass):
- docs.yaml exists at the repository root (site-wide Git Sync mapping)
- documentation/.gitbook.yaml points at README.md + SUMMARY.md
- every relative .md link inside documentation/ resolves on disk
- every page in documentation/ is reachable from SUMMARY.md or README.md
- no secret-shaped filenames (.env, .credentials.json, id_rsa, ...)

Place at <repo>/documentation/scripts/verify-docs.py; run from repo root:
    python3 documentation/scripts/verify-docs.py

Adjust ROOT/SPACE constants if the space lives elsewhere.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPACE = ROOT / "documentation"
SUMMARY = SPACE / "SUMMARY.md"
GITBOOK = SPACE / ".gitbook.yaml"
DOCS_YAML = ROOT / "docs.yaml"

FORBIDDEN_NAMES = {
    ".env",
    ".env.local",
    ".credentials.json",
    "id_rsa",
    "id_ed25519",
}

LINK = re.compile(r"\]\(([^)#]+?\.md)\)")


def fail(message: str) -> int:
    print(f"FAIL: {message}", file=sys.stderr)
    return 1


def main() -> int:
    if not DOCS_YAML.is_file():
        return fail("missing docs.yaml at repository root (GitBook site mapping)")
    if not GITBOOK.is_file():
        return fail("missing documentation/.gitbook.yaml")
    if not SUMMARY.is_file():
        return fail("missing documentation/SUMMARY.md")
    if not (SPACE / "README.md").is_file():
        return fail("missing documentation/README.md")

    yaml_text = GITBOOK.read_text(encoding="utf-8")
    if "readme: README.md" not in yaml_text or "summary: SUMMARY.md" not in yaml_text:
        return fail(".gitbook.yaml must declare README.md and SUMMARY.md")

    # 1. Every link target must exist.
    missing: list[str] = []
    listed: set[Path] = set()
    for md in sorted(SPACE.rglob("*.md")):
        for match in LINK.finditer(md.read_text(encoding="utf-8")):
            rel = match.group(1).strip()
            path = (md.parent / rel).resolve()
            if not path.is_file():
                missing.append(f"{md.relative_to(ROOT)} -> {rel}")
            else:
                listed.add(path)

    if missing:
        return fail("broken links:\n  " + "\n  ".join(sorted(set(missing))))

    # 2. Every content page must be reachable from SUMMARY.md or README.md.
    entry = SUMMARY.read_text(encoding="utf-8") + (SPACE / "README.md").read_text(
        encoding="utf-8"
    )
    unreachable: list[str] = []
    for md in sorted(SPACE.rglob("*.md")):
        if md.name in {"README.md", "SUMMARY.md"}:
            continue
        if md not in listed and str(md) not in entry:
            unreachable.append(str(md.relative_to(ROOT)))

    if unreachable:
        return fail(
            "pages not linked from SUMMARY.md/README.md:\n  " + "\n  ".join(unreachable)
        )

    # 3. No secret-shaped filenames.
    secrets = [
        str(p.relative_to(ROOT))
        for p in SPACE.rglob("*")
        if p.name in FORBIDDEN_NAMES
    ]
    if secrets:
        return fail("forbidden files present:\n  " + "\n  ".join(secrets))

    pages = len(list(SPACE.rglob("*.md")))
    print(
        f"OK: docs.yaml + space structure valid; {pages} pages; "
        "no broken links; nothing unreachable."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
