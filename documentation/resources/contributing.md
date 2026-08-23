# Contributing to documentation

The docs live **in the product repository** under `documentation/` and sync to GitBook via Git Sync. Markdown is the source of truth — the published site is a rendering of this directory.

## Layout

```
docs.yaml                  # site-wide Git Sync mapping (repo root)
documentation/
├── .gitbook.yaml          # space structure: README + SUMMARY
├── README.md              # welcome page (space home)
├── SUMMARY.md             # table of contents — every page must be listed
├── *.md                   # getting started + usage pages
├── architecture/          # per-crate technical structure
├── reference/             # tools/, proto, schema, env
├── surfaces/ web3/ ops/ quality/ resources/
└── scripts/verify-docs.py # link/structure checker
```

## Rules

1. **Ground every claim in the tree.** Cite source paths (`crates/...`, `file:line`) for behavior. If a flag, crate, or provider ID isn't on disk, it doesn't get documented.
2. **Every page ends with a Related section** linking its neighbors. Cross-link generously; orphan pages fail review.
3. **SUMMARY.md is mandatory membership.** New pages must be added to it, or `verify-docs.py` fails with "not linked from SUMMARY.md".
4. **Audience split**: customer-facing content lives in Getting started / Usage / Help; technical structure lives under Architecture / Tool reference / API reference / Development. Keep the two voices distinct — no house policy dumps in customer pages.
5. **No secrets**: never commit `.env`, `.credentials.json`, tokens, or real keys. Examples use placeholders.
6. **Match tone**: direct, evidence-first prose. Code blocks over vague description. Tables for parameters and flags.

## Verify before pushing

```bash
python3 documentation/scripts/verify-docs.py
```

Checks: `docs.yaml` + `.gitbook.yaml` present, all relative `.md` links resolve, every page reachable from SUMMARY/README, no secret-shaped filenames.

## Git Sync flow

- The mapping is declared once in `docs.yaml` (site → space → `./documentation`).
- Commits to the synced branch publish automatically through GitBook's GitHub sync; PR previews are available when the integration is enabled.
- Move a page = move the file **and** update `SUMMARY.md` in the same commit, or the mapped space can render empty mid-move.

## Related

- [GitBook monorepos](https://gitbook.com/docs/docs-as-code/git-sync/monorepos) — how site-wide sync maps directories
- [Architecture overview](../architecture/overview.md) — what the technical half documents
