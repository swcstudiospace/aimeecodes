# Concurrent Grok handoff race (2026-07-29)

## What happened

Three near-identical Grok→Hermes handoffs (`…95a5fb76`, `…998990bd`, `…4d65ea30`)
targeted the same goal: upgrade `grok-hermes-connector` with Tasks + skills + prompts.
Agents race-wrote `server.py`, `tasks.py`, `skills.py`, `audit.py`, leaving:

- Import mismatches (`AuditLogger` vs `AuditLog`, `TasksExtension` kwargs)
- Two Task designs: HandoffStore-backed extension vs separate TaskStore dual-write
- Read-only freezes on modules mid-edit
- Tests failing while `/health` still served old code until restart/install

## Recovery recipe (worked)

1. Stop inventing a third `server.py`. Treat the largest/latest `server.py` as the
   integration surface; list every symbol it imports.
2. Implement **compat** APIs so both call styles work where cheap
   (`TasksExtension(store)` *or* `TasksExtension(task_store, handoff_lookup=…)`).
3. Skill catalog: `list_skills` → dict `{count,total,skills}`; `get_info` → SkillInfo;
   bodies via `read_skill_md`.
4. `TaskStore` returns **attribute records** (dataclass), not bare dicts — server
   uses `rec.taskId` / `rec.status`.
5. `chmod u+w` on frozen files; one `pytest -q` (21 passed); `uv pip install -e .`;
   `systemctl restart grok-hermes-connector`; confirm `/health` version + extension.

## Prevention advice to Grok

- One `handoff_to_hermes` per goal; poll `get_task` instead of re-firing.
- If parallel jobs are unavoidable, handoffs should target **disjoint paths**.

## Related

Connector docs: `/root/src/repos/grok-hermes-connector/docs/UPGRADE_v0.3.md`
