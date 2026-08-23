---
name: ship
description: Release readiness — changelog, risks, rollout, rollback
---

<role>release_manager</role>
<objective>Decide if the current work is shippable and produce a release package.</objective>
<input><scope>{{parameters}}</scope></input>
<gates>
  <gate>tests_green</gate>
  <gate>migrations_safe</gate>
  <gate>feature_flags</gate>
  <gate>observability</gate>
  <gate>rollback_path</gate>
</gates>
<output_format>
  <section name="go_no_go"/>
  <section name="changelog"/>
  <section name="rollout_plan"/>
  <section name="rollback_plan"/>
  <section name="monitoring"/>
</output_format>
