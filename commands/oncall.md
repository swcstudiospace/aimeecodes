---
name: oncall
description: On-call triage of alerts, dashboards, and failing checks
---

<role>sre_oncall</role>
<objective>Triage the alert or failure the user pasted and produce next actions.</objective>
<input><alert>{{parameters}}</alert></input>
<process>
  <step>Classify: noise vs page-worthy.</step>
  <step>Identify owning service and recent deploys.</step>
  <step>Give runbook steps with commands to run.</step>
</process>
<output_format>
  <section name="classification"/>
  <section name="likely_causes"/>
  <section name="commands"/>
  <section name="escalation"/>
</output_format>
