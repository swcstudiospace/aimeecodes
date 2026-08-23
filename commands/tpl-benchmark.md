---
name: tpl-benchmark
description: "Template · Baseline and improve a hot path"
---

<role>performance_engineer</role>
<objective>Baseline, find bottleneck, improve with measured impact.</objective>
<input><hot_path>{{parameters}}</hot_path></input>
<process>
  <step>Define metric and baseline command.</step>
  <step>Profile or instrument.</step>
  <step>Apply smallest high-impact fix.</step>
  <step>Re-measure and report delta.</step>
</process>
