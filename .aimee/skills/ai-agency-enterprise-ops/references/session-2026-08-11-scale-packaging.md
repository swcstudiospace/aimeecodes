# Session learnings — scale-30 + dual-write + cockpit packaging (2026-08-11)

## Scale

- **30 agents / 12 teams / 10 workflows** live on AgentOS `:7777`.
- New ops agents must match original archetype: ≥5–6KB persona packs, domain+agent playbooks, dedicated `*_ops` toolbelts, schemas — never ship thin stubs.
- Avoid skill folder name collisions between `skills/ops/` and `skills/agents/` (e.g. use `catalog-ops-agent-playbook`).
- Profile skill names must exist in index (`compliance-ads-claims`, not alias `claims-compliance`).

## KIP dual-write

- Authoritative brain: `anda-nexus` `:8091`.
- `kip_memory/dual_write.py` + `ensure_agency_schema_remote` for types + propositions.
- Remote search: `SEARCH CONCEPT "q" LIMIT n`.
- Daily maintenance script/timer + analytics SQLite (not the brain).
- See `references/kip-dual-write-and-analytics.md`.

## Cockpit packaging

- App: `agency-cockpit/` — skill **`agency-desktop-genui`** + `references/multi-target-packaging.md`.
- Verified: PWA `dist/sw.js`; Linux deb/rpm/AppImage; binary under `src-tauri/target/release/`.
- Linux AppImage needs **xdg-utils**; webkit2gtk-4.1 for compile.
- Mobile: `tauri android|ios init` generates `src-tauri/gen/` (not committed).

## Interop ports

| Service | Port |
|---------|------|
| AgentOS | 7777 |
| Drop | 7788 |
| Hermes bridge | 7790 |
| Anda nexus | 8091 |
| Cockpit Vite | 1420 |
