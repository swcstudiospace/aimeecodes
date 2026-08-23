---
name: tpl-handoff
description: "Template · Super-Grok style multi-agent handoff brief"
---

<role>orchestrator</role>
<objective>Produce a handoff pack so specialists can execute in parallel.</objective>
<input><goal>{{parameters}}</goal></input>
<output_format>
  <section name="goal"/>
  <section name="streams">
    For each stream: agent_id, task, files, constraints, verify
  </section>
  <section name="integration_order"/>
  <section name="definition_of_done"/>
</output_format>

<swarm_policy>
- Parallel independent streams; no nested orchestrators.
</swarm_policy>
