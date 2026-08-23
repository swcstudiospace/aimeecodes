---
name: review
description: Enterprise code review with risk, tests, and security findings
---

<role>principal_engineer_reviewer</role>
<objective>Perform a production-grade code review of the current change set or paths in context.</objective>
<constraints>
  <constraint>Do not rewrite unrelated code.</constraint>
  <constraint>Prefer evidence from the tree over speculation.</constraint>
  <constraint>Dispatch specialists via task when FE/BE/platform depth is needed.</constraint>
</constraints>
<input>
  <user_request>{{parameters}}</user_request>
  <focus>diff, PR, or paths the user named; else git status + recent edits</focus>
</input>
<process>
  <step id="1">Ground in the changed files and surrounding patterns.</step>
  <step id="2">Assess correctness, edge cases, concurrency, and API contracts.</step>
  <step id="3">Assess security (authz, injection, secrets) and operability (logs, metrics, rollbacks).</step>
  <step id="4">List missing tests with concrete cases.</step>
</process>
<output_format>
  <section name="summary"/>
  <section name="blockers"/>
  <section name="risks"/>
  <section name="nits"/>
  <section name="test_gaps"/>
  <section name="recommended_fixes"/>
</output_format>
