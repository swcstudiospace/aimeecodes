# PromptWise UGC + GitHub CI (session 2026-08-11)

## PromptWise

- Site: https://www.promptwise.com · App: https://app.promptwise.com
- Features used in agency narrative: **UGC Factory**, Influencer Studio, Flows, Wise assistant
- Models marketed on site include Kling / Seedance / Nanobanana family (credits-based)
- **No durable public OpenAPI** found; FAQ mentions MCP interest — treat API as optional

### Integration contract

```text
promptwise_build_ugc_brief(...)
  → artifact tmp/creatives/promptwise/brief_*.json
  → wise_prompt string for paste into Wise / prompt bar

promptwise_run_ugc_job(..., open_browser=True)
  → if PROMPTWISE_API_KEY+BASE: best-effort POST (may 404 until official paths)
  → else Hermes bridge browser open app.promptwise.com
  → hitl=True for credit spend when UI-only

Fallback: tools/fal_tools.generate_ugc_avatar_video
```

### Env

```bash
PROMPTWISE_APP_URL=https://app.promptwise.com
PROMPTWISE_BROWSER_ENABLED=1
PROMPTWISE_API_KEY=
PROMPTWISE_API_BASE=
HERMES_BRIDGE_URL=http://127.0.0.1:7790
```

### Operator once-per-host

1. `systemctl status hermes-bridge` (or start bridge)
2. Log into PromptWise in the bridge browser profile
3. Agent runs job → human confirms generation → save MP4 under `tmp/creatives/promptwise/`
4. Linear dual-write + no Meta/TikTok spend without HITL vault

### Agents wired

- `creative_director` toolbelts include `promptwise`
- `ads_creative_ops` toolbelts include `promptwise` + `creative_ops` (which also embeds promptwise+fal tools)

## GitHub Actions

| File | Purpose |
|------|---------|
| `.github/workflows/ci.yml` | umbrella `workflow_call` → backend + cockpit |
| `.github/workflows/ci-backend.yml` | compile package roots + offline smoke |
| `.github/workflows/ci-cockpit.yml` | npm ci + build:web; Tauri deb optional |

### Backend smoke essentials

- `XAI_API_KEY=missing-ci-placeholder` so factory constructs
- Disable bridge/brain/analytics env flags for offline
- `LINEAR_GITHUB_LINK=0` so CI does not create GH issues
- Assert `promptwise` in `TOOLBELTS`
- `PROFILES` is a **list** — use `profile.key`

### Failures seen and fixes

1. **drop_server/mcp_app.py** mangled `drop_roster` (`return { return {`) → compileall failed → rewrite clean roster JSON
2. **envutil** read `/root/.config/hermes-linear/connector.env` → PermissionError on GHA runner → home-relative paths + OSError around `is_file`/`read_text`; force project `.env` for LINEAR_* still applies on the VPS
3. Path-filtered leaf workflows cancelled when umbrella runs — expected; green = umbrella jobs

### Verify locally (mirror CI)

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate
export PYTHONPATH=. XAI_API_KEY=missing-ci-placeholder AGENCY_GROK_MODEL=grok-4.5
export AGENCY_DISABLE_HERMES_BRIDGE=1 AGENCY_DISABLE_ANDA_BRAIN=1
export AGENCY_DISABLE_ANDA_KNOWLEDGE=1 AGENCY_DISABLE_ANALYTICS=1 LINEAR_GITHUB_LINK=0
python -c "from tools.toolbelts import TOOLBELTS; assert 'promptwise' in TOOLBELTS"
python -m compileall agents app tools teams workflows scripts evals drop_server hermes_bridge kip_memory -q
```

Cockpit: `cd agency-cockpit && npm ci && npm run build:web`
