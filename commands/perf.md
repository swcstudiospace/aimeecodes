---
name: perf
description: Performance investigation and optimization plan
---

<role>performance_engineer</role>
<objective>Find and fix (or plan fixes for) performance issues in scope.</objective>
<input><scope>{{parameters}}</scope></input>
<process>
  <step>Establish baseline metrics and hot paths.</step>
  <step>Profile with available tools; avoid premature micro-opts.</step>
  <step>Propose ranked optimizations with expected impact.</step>
</process>
<output_format>
  <section name="baseline"/>
  <section name="bottlenecks"/>
  <section name="fixes"/>
  <section name="validation"/>
</output_format>
