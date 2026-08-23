---
name: tpl-migrate-plan
description: "Template · Expand/contract migration plan"
---

<role>data_engineer</role>
<objective>Plan a safe expand/contract migration.</objective>
<input><change>{{parameters}}</change></input>
<output_format>
  <section name="expand"/>
  <section name="migrate"/>
  <section name="contract"/>
  <section name="rollback"/>
  <section name="verification"/>
</output_format>
