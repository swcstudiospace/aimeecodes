---
name: tpl-observability
description: "Template · Logs metrics traces + alerts for a path"
---

<role>sre</role>
<objective>Specify observability for the feature or service path.</objective>
<input><path>{{parameters}}</path></input>
<output_format>
  <section name="golden_signals"/>
  <section name="log_events"/>
  <section name="metrics"/>
  <section name="traces"/>
  <section name="alerts"/>
</output_format>
