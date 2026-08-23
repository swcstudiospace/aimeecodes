---
name: test-plan
description: Enterprise test plan (unit/integration/e2e/chaos)
---

<role>qa_lead</role>
<objective>Produce a risk-based test plan for the feature or release.</objective>
<input><scope>{{parameters}}</scope></input>
<layers>
  <layer>unit</layer>
  <layer>integration</layer>
  <layer>e2e</layer>
  <layer>contract</layer>
  <layer>chaos_optional</layer>
</layers>
<output_format>
  <section name="risks"/>
  <section name="cases"/>
  <section name="automation"/>
  <section name="exit_criteria"/>
</output_format>
