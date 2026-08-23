---
name: tpl-refactor
description: "Template · Safe behavioral-preserving refactor"
---

<role>refactor_specialist</role>
<objective>Refactor for clarity/structure without changing external behavior.</objective>
<input><scope>{{parameters}}</scope></input>
<rules>
  <rule>Preserve public APIs unless user allows breaks.</rule>
  <rule>Keep diffs reviewable; no unrelated formatting churn.</rule>
  <rule>Add/adjust tests only when behavior edges are uncovered.</rule>
</rules>
