# Repos and bootstrap

## Layout (this host)

```text
/root/src/repos/hermes-linear-connector/
/root/src/repos/hermes-engineering-board/
~/.config/hermes-linear/config.yaml      # no secrets
~/.config/hermes-linear/connector.env    # LINEAR_API_KEY, mode 600
~/.config/hermes-eng-board/config.yaml   # autopilot + gates
~/.hermes/kanban/boards/eng/kanban.db    # after bootstrap
~/.hermes/eng-board/sessions/            # Autopilot work units
~/.hermes/plugins/hermes_eng_board -> …/plugin/hermes_eng_board
~/.hermes/skills/software-development/enterprise-engineering/
```

## Bootstrap

```bash
bash /root/src/repos/hermes-engineering-board/scripts/bootstrap_workspace.sh          # dry-run
bash /root/src/repos/hermes-engineering-board/scripts/bootstrap_workspace.sh --apply
bash /root/src/repos/hermes-engineering-board/scripts/bootstrap_profiles.sh --apply   # optional
hermes kanban boards list    # eng current
hermes project list          # engineering
cd /root/src/repos/hermes-engineering-board && uv pip install -e .
# restart gateway/CLI so plugin hooks load
```

## Dev / verify

```bash
cd /root/src/repos/hermes-linear-connector && uv sync
uv run pytest -m "not live and not hermes" -q
set -a; source ~/.config/hermes-linear/connector.env; set +a
uv run hermes-linear-connector health
uv run pytest -m "live or hermes" -v   # [hermes-sync-test] + cleanup; Done needs dod_pass

cd /root/src/repos/hermes-engineering-board && uv sync && uv run pytest -q
uv run pytest -m hermes -v
```

## Processes

```bash
uv run python -m hermes_linear_connector.api_main      # ensure HTTP
uv run python -m hermes_linear_connector.poller_main   # long poll
python scripts/run_autopilot_once.py --sweep          # from eng-board repo
# deploy/hermes-linear-poller.service stub in connector repo
# This host: ensure API is systemd hermes-linear-api on 127.0.0.1:8792
# (do NOT use :8788 — that is grok-hermes-connector).
```

Gateway must be up for kanban dispatcher ticks. See also `references/autopilot-and-gates.md`.
