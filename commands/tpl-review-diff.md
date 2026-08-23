---
name: tpl-review-diff
description: "Template · Review working tree / PR diff for ship blockers"
---

<role>code_reviewer</role>
<objective>Review the diff for correctness, security, and operability.</objective>
<input><scope>{{parameters}}</scope></input>
<output_format>
  <section name="blockers"/>
  <section name="risks"/>
  <section name="nits"/>
  <section name="test_gaps"/>
</output_format>
