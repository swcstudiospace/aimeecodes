# ClippyOS-shaped handbook

Standalone TanStack Start product (emerald `#10B981`, IBM Plex Mono). Facts only from that repo — not `ai-clipping` or other sibling trees.

## Catalog contract

- Space: `documentation/` + `SUMMARY.md` + `.gitbook.yaml`
- After edits: `python3 scripts/sync-docs-catalog.py` → `src/lib/docs/generated.ts`
- One-shot dump (optional): `scripts/generate-docs.py` / `scripts/write-handbook.py`
- Helpers: `src/lib/docs/catalog.ts` (`hrefFor`, `slugFromPath`, `getDoc`, `docGroups`, `neighbors`, `extractToc`, `searchDocs`)
- Renderer: `src/lib/docs/markdown.tsx`

Welcome slug is `""` → `/docs`. Nested slugs are paths without `.md`.

## Routes

```text
src/routes/docs.tsx         # layout
src/routes/docs.index.tsx   # /docs
src/routes/docs.$.tsx       # /docs/$
```

`slugFromPath(pathname)` strips `/docs/` — do not depend on `_splat` vs `*`.

## Landing surfaces that must link `/docs`

Header **Docs** · hero **Documentation** · `#docs` handbook cards · FAQ “open the handbook” · footer Documentation / Quickstart / Integrations / **AGL-3.0**.

## License

`LICENSE` = Autonogrammer General License 3.0 for Spectrum Web Co LLC (`LicenseRef-AGL-3.0`). In-app page `/docs/legal/agl-3.0`. `package.json` `"license": "LicenseRef-AGL-3.0"`.

## QA pages

- `/docs` — sidebar groups + welcome article
- `/docs/getting-started/local-setup` and `/docs/operators/common-errors` — real steps + error codes
- `/docs/legal/agl-3.0`
- Landing Docs click → `/docs`
