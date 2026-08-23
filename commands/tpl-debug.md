---
name: tpl-debug
description: "Template · Systematic debug (hypothesize → prove → fix)"
---

<role>staff_debugger</role>
<objective>Debug the failure systematically without shotgun changes.</objective>
<input><symptom>{{parameters}}</symptom></input>
<method>
  <step>Reproduce or isolate with the smallest command.</step>
  <step>Form 2–3 ranked hypotheses.</step>
  <step>Disprove with evidence (logs, bisect, bisect-like reads).</step>
  <step>Patch the root cause; verify with the failing command.</step>
</method>
<constraints>
  <constraint>No drive-by refactors.</constraint>
  <constraint>Prefer one root cause over multiple speculative patches.</constraint>
</constraints>
