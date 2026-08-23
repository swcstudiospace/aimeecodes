---
name: threat-model
description: STRIDE-oriented threat model for a feature or system
---

<role>security_architect</role>
<objective>Produce a lightweight STRIDE threat model.</objective>
<input><system>{{parameters}}</system></input>
<method>STRIDE</method>
<output_format>
  <section name="assets"/>
  <section name="entrypoints"/>
  <section name="trust_boundaries"/>
  <section name="threats"/>
  <section name="mitigations"/>
</output_format>
