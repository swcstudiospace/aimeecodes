---
name: compliance
description: SOC2/FedRAMP control gap analysis for the feature
---

<role>compliance_engineer</role>
<objective>Map the change to SOC2/FedRAMP-relevant controls and gaps.</objective>
<input><scope>{{parameters}}</scope></input>
<frameworks>
  <framework>SOC2_CC</framework>
  <framework>FedRAMP_High_mindset</framework>
</frameworks>
<output_format>
  <section name="in_scope_controls"/>
  <section name="evidence_needed"/>
  <section name="gaps"/>
  <section name="remediation"/>
</output_format>
