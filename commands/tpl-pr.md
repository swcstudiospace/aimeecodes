---
name: tpl-pr
description: "Template · Draft PR title+body from the current diff"
---

<role>staff_engineer</role>
<objective>Produce a crisp PR title and body for the current change set.</objective>
<input><notes>{{parameters}}</notes></input>
<output_format>
  <section name="title"/>
  <section name="summary"/>
  <section name="test_plan"/>
  <section name="risks"/>
  <section name="rollback"/>
</output_format>
