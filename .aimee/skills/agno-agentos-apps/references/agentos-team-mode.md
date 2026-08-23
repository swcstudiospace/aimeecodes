# AgentOS Team.mode serialization (validated 2026-08)

## Symptom

```http
GET /teams
{"detail":"'str' object has no attribute 'value'"}
```

Agents and workflows may still list fine. Only team listing/serialization fails.

## Cause

AgentOS schema code does:

```python
mode=team.mode.value if team.mode else None
```

If you constructed:

```python
Team(mode="coordinate", ...)
```

then `team.mode` is a plain `str` and `.value` raises.

## Fix

```python
from agno.team import Team
from agno.team.mode import TeamMode

Team(
    name="Research Team",
    mode=TeamMode.coordinate,  # not "coordinate"
    model=...,
    members=[...],
)
```

Enum members: `coordinate`, `route`, `broadcast`, `tasks`.

## Verify

```python
assert all(hasattr(t.mode, "value") for t in agent_os.teams)
# HTTP:
# curl -s localhost:7777/teams  → JSON list of teams
```

## Note

`mode="coordinate"` may still *run* teams in-process; the bug is OS API serialization. Always use the enum for AgentOS-hosted apps.
