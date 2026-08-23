# GitBook monorepo Git Sync — confirmed facts

Source: GitBook docs "Monorepos" (docs-as-code/git-sync/monorepos), scraped 2026-08-22 into docs-gitbook corpus.

## Site-wide Git Sync mechanics

- The site's **Project directory** sets where Git Sync is installed; `docs.yaml` lives there (repo root if left empty).
- `docs.yaml` shape (validated against `GitSyncSiteConfig`):

```yaml
$schema: https://api.gitbook.com/openapi.yaml#/components/schemas/GitSyncSiteConfig
site:
  structure:
    - type: space
      key: documentation
      title: Documentation
      path: documentation
      content:
        directory: ./documentation
```

- Each space's directory holds its own `.gitbook.yaml`, `README.md`, `SUMMARY.md`, and assets. **GitBook does not share content or assets between mapped directories** — duplicate assets per space or reorganize.
- Path semantics in mapping: `./docs` = inside Project directory; `/docs` = from repo root; `docs` = same as `./docs`.
- Moving a mapped directory: move files + update `content.directory` **in the same commit**, or the space can import empty.
- Individual-space Git Sync is for spaces needing a different repo/branch; remove from site mapping first.

## Space structure files

`.gitbook.yaml` inside the space:

```yaml
root: ./
structure:
  readme: README.md
  summary: SUMMARY.md
```

`SUMMARY.md`: `# Table of contents`, welcome entry, then `## Group` headings each followed by `* [Title](path.md)` lines. Every referenced file must exist or sync fails.

## Working reference implementation

aimeecodes repo (Rust workspace, 2026-08):

- Root: `docs.yaml` mapping one space `documentation` → `./documentation`.
- `documentation/`: README (welcome + org table + audience split), SUMMARY with 10 groups (Getting started, Usage, Product surfaces, WEB3, Operations, Help, Architecture, Tool reference — 16 per-tool pages, API reference — proto/schema/env, Development).
- `documentation/scripts/verify-docs.py`: fails on broken relative `.md` links anywhere in the space, pages not reachable from SUMMARY/README, forbidden filenames (`.env`, `.credentials.json`, `id_rsa`, …). Pattern worth reusing for any docs space.
- Porting audit hits found in practice: `runtime/streaming.md` → `architecture/streaming.md` path moves, dropped `pwa-delivery.md` sibling, org-page references (`../org/…`) that don't exist in the new repo.

## Renderer clone note

`GitbookIO/gitbook` (the open-source renderer, GPLv3) is a *published-site renderer*, not the content. "Clone GitBook" for docs purposes = content space + Git Sync, not vendoring that app. A clone may exist as a sibling repo for self-hosted preview; keep it out of the product repo.

## Placement default

When offering placement options (standalone org docs repo / fresh repo / space inside the product repo), the user picked the **product-repo monorepo** (aimeecodes, 2026-08). Default to that unless told otherwise.

## Porting an existing docs space — validated workflow

Copy first preserving subpaths (`architecture/`, `ops/`, …) so relative links survive, then audit BEFORE hand-fixing anything:

```bash
python3 - <<'EOF'
import re
from pathlib import Path
for md in Path(".").rglob("*.md"):
    for m in re.finditer(r"\]\(([^)#]+\.md)\)", md.read_text()):
        if not (md.parent / m.group(1)).resolve().is_file():
            print(f"{md} -> {m.group(1)}")
EOF
```

Fix classes seen in practice, cheapest first: (a) path-depth — pages moved one directory deeper need a `../` prefix; bulk-fix programmatically against a known top-level name set rather than hand-editing each link; (b) links OUTSIDE the space — replace with a prose mention of the repo path or the nearest in-space equivalent; (c) stale provenance — grep ported text for the old repo name and old verify-script paths; copied "Verify" sections often point at the old home.

## Per-tool reference pages (source-grounding technique)

When documenting a coding-agent CLI's built-in tools, derive every fact from source, never from memory:

- Input struct = parameter table. Capture serde aliases (legacy field names), `#[serde(rename)]` wire names (`"-B"`, `"-n"`), defaults (`#[serde(default)]`, `default_true`), and `deny_unknown_fields`.
- The tool-description files (`descriptions/*.md` or equivalent) are the LLM-facing contract — lift behavior rules from those; they are the canonical wording.
- Permission posture mirrors the policy-conversion match arms: which variants gate Read/Write/Execute/Fetch, which return `None` (ungated). Note yield-on-call and stdout-only catalog flags where defined.
- Page anatomy: Purpose → Parameters → Example JSON call → Behavior → Errors → Permissions → Related.
- Catalog hub page carries: tool × input-type × permission table, name-normalization rules (case-insensitive, legacy aliases), schema-generation notes, rules for adding a new tool.
