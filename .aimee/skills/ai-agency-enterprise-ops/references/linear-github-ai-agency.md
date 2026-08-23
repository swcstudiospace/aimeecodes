# Linear ↔ GitHub: ai-agency (not Agent Runtime)

## Canonical targets
| Surface | Value |
|---------|--------|
| GitHub | https://github.com/swcstudiospace/ai-agency |
| Linear org | spectrumwebco |
| Team | **SWC** |
| Project | AI Dropshipping Agency |
| Wrong default | `swcstudiospace/agent_runtime` (Agent Runtime) |

## Symptom
Linear issue attachments show `github.com/.../agent_runtime/issues/N` for agency work (product rank, bootstrap, discovery).

## Fix (agency issues SWC-60…66 pattern)
1. Force correct Linear key from project `.env` (overwrite shell):
   - `tools/envutil.py` forces `LINEAR_*` from `/root/src/repos/ai-agency/.env`
2. GraphQL: list issue attachments; `attachmentDelete` any URL containing `agent_runtime`
3. `gh` create issue on `swcstudiospace/ai-agency` with Linear identifier in title
4. `attachmentLinkGitHubIssue(issueId, url)` (fallback `attachmentLinkURL`)
5. Comment on Linear noting the correction

## Ongoing code path
- `create_linear_issue` → `link_issue_to_github_repo` (`tools/linear_tools.py`)
- Env: `LINEAR_GITHUB_REPO=swcstudiospace/ai-agency`, `LINEAR_GITHUB_LINK=1`
- Disable auto GH: `LINEAR_GITHUB_LINK=0`

## Workspace UI (human, once)
Linear → Settings → Integrations → GitHub (spectrumwebco):
- Prefer **ai-agency** for team SWC
- Uncheck agent_runtime as default for agency project

## Git push notes
- Repo may start empty / LICENSE-only remote → merge unrelated histories then push `main`
- Never stage `.env`; `.gitignore` covers `tmp/`, cockpit `node_modules`/`target/`, `kip_memory/data/`

## Debug org mismatch
If `linear_status` or GraphQL returns **swcstudio** / SPE instead of **spectrumwebco** / SWC:
```bash
# shell may hold another LINEAR_API_KEY
python -c "from tools.envutil import load_dotenv_files; load_dotenv_files(); import os; print(os.environ['LINEAR_API_KEY'][:12])"
# expect spectrumwebco
```
Then call GraphQL `{ viewer { organization { urlKey } } teams { nodes { key } } }`.
