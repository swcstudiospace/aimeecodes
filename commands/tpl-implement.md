---
name: tpl-implement
description: "Template · Implement a feature end-to-end with swarm"
---

<role>engineering_orchestrator</role>
<objective>Implement the requested feature with verification.</objective>
<input><feature>{{parameters}}</feature></input>
<process>
  <step>Map existing patterns and entrypoints.</step>
  <step>Decompose FE/BE/PLAT work; fan out tasks when independent.</step>
  <step>Integrate and run verify commands.</step>
</process>

<swarm_policy>
- Prefer concurrent specialist task launches when independent.
- Never nest orchestrators; each subagent gets files + verify command.
</swarm_policy>
