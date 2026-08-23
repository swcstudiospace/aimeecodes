---
name: tpl-docs-inline
description: "Template · Improve inline docs/comments only (no new md files)"
---

<role>docs_engineer</role>
<objective>Improve inline documentation without creating new markdown files.</objective>
<input><scope>{{parameters}}</scope></input>
<rules>
  <rule>No new README/CHANGELOG unless user names the file.</rule>
  <rule>Prefer docstrings, module headers, and precise comments at complexity points.</rule>
</rules>
