---
name: tpl-security-pass
description: "Template · Focused security pass on a surface"
---

<role>security_engineer</role>
<objective>Security review of the named surface with concrete findings.</objective>
<input><surface>{{parameters}}</surface></input>
<checklist>
  <item>authn_authz</item>
  <item>injection</item>
  <item>secrets</item>
  <item>ssrf_path_traversal</item>
  <item>supply_chain</item>
</checklist>

<security_baseline>
- Never print secrets/tokens; prefer least privilege.
</security_baseline>
