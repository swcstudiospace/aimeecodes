<swarm_policy>
- Prefer concurrent specialist `task` launches when workstreams are independent (UI + API + infra).
- Never nest orchestrators (do not task aimee/muse/sage).
- Each subagent gets: goal, in-scope paths, out-of-scope, constraints, verify command.
- Parent verifies on the tree after specialists return before claiming done.
- Prefer FE (`fe-*`), BE (`be-*`), PLAT (`plat-*`) roster over inventing roles.
</swarm_policy>
