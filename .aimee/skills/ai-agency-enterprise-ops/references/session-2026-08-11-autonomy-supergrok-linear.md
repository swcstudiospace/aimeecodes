# Session notes — SuperGrok, SWC Linear, product rank, bi-daily cron

## Linear (agency)

- Org **spectrumwebco**, team **SWC** (UUID in `.env` `LINEAR_TEAM_ID`), project **AI Dropshipping Agency** (`LINEAR_PROJECT_ID`).
- Do **not** use SPE state UUIDs from eng config — refresh states from GraphQL `team(id){ states { nodes { id name type } } }`.
- `tools/linear_tools.py` defaults: `team_key=SWC`, auto `project_id` from env.
- Quote multi-word env values in `.env`.

## SuperGrok

- Live model for agents: **`grok-4.5`** via Hermes OAuth pool (no console key required when OAuth works).
- Status/login: `python -m tools.xai_oauth_pkce status|login`.
- Smoke: POST `/v1/chat/completions` model `grok-4.5`.

## Product rank (operator path)

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate
set -a && source .env && set +a
export PYTHONPATH=. AGENCY_GROK_MODEL=grok-4.5 PYTHONUNBUFFERED=1
python -u -m scripts.autonomous_product_rank --niche "…" --processor pro
# artifacts: tmp/runs/product_rank_*.{json,md}
# dual-write GO/TEST → Linear SWC project (load_dotenv_files inside script)
```

Example top result (2026-08-11 desk mobility): Fold-Flat Adjustable Aluminum Laptop Stand GO ~78.3 @ $49.99 CM%~34.5% (SWC-63–66).

## Bi-daily discovery

- Cron name `agency-product-discovery-bidaily`, `0 8,20 * * *` UTC, workdir repo root.
- Rotates niches; no ad spend.

## Keys (discovery vs sell)

- **Required for rank:** SuperGrok OAuth, Parallel, Linear.
- **Not required for rank:** Fal, Shopify, Meta, TikTok (stubs until live store/ads).

## Cockpit packaging (cross-ref)

- `agency-cockpit/`: web PWA + Tauri desktop (deb/rpm/AppImage) + mobile init scripts.
- AppImage needs `xdg-utils` on Linux build host.
- Skill: `agency-desktop-genui` → `references/multi-target-packaging.md`.
