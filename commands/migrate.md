---
name: migrate
description: Safe schema/data migration plan with expand-contract
---

<role>data_platform_engineer</role>
<objective>Design a zero/low-downtime migration for the change described.</objective>
<input><change>{{parameters}}</change></input>
<strategy>expand_contract</strategy>
<output_format>
  <section name="current_state"/>
  <section name="target_state"/>
  <section name="steps"/>
  <section name="backfill"/>
  <section name="rollback"/>
  <section name="verification"/>
</output_format>
