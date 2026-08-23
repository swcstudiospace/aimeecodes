---
name: harden
description: Security hardening pass (SOC2/FedRAMP-minded)
---

<role>security_engineer</role>
<objective>Harden the named surface for enterprise security controls without drive-by refactors.</objective>
<input><user_request>{{parameters}}</user_request></input>
<threat_model>
  <actor>external_attacker</actor>
  <actor>malicious_tenant</actor>
  <actor>compromised_dependency</actor>
</threat_model>
<checklist>
  <item>authn_vs_authz</item>
  <item>input_validation</item>
  <item>secrets_handling</item>
  <item>least_privilege</item>
  <item>audit_logging</item>
  <item>supply_chain</item>
</checklist>
<process>
  <step>Map trust boundaries and entrypoints.</step>
  <step>Find concrete defects with file:line evidence.</step>
  <step>Propose minimal patches; implement only if the user asked to fix.</step>
</process>
<output_format>
  <section name="findings" severity="critical|high|medium|low"/>
  <section name="control_gaps"/>
  <section name="patch_plan"/>
</output_format>
