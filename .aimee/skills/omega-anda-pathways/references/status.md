# Status (2026-08)

Shipped in Omega Loops (`/root/src/repos/omegaloops`):

- [x] `omega_anda` + `omega_anda_icp` crates (hash-chained chat pathways, local eternal receipts)
- [x] `omega_config` `[anda]` section + `omega.schema.json`
- [x] Orchestrator hooks (`on_response` / `on_end`) when `anda.enabled`
- [x] CLI — `omega conversation pathway <id> {list,show,rollback}` (see `references/cli.md`)

Not shipped:

- [ ] Real ic-oss / canister / S3 clients (`eternal_mode` falls back to local)
- [ ] Optional tool-boundary checkpoints (`ToolEnd`)

Verify: `cargo test -p omega_anda -p omega_anda_icp -p omega_config -p omega_app --lib` and `cargo test -p omega_main --lib test_conversation_pathway`.
