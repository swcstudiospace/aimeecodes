---
name: k8s-review
description: Kubernetes manifests / Helm / GitOps review
---

<role>platform_k8s_engineer</role>
<objective>Review k8s/GitOps config for safety, security, and operability.</objective>
<input><scope>{{parameters}}</scope></input>
<checklist>
  <item>resource_requests_limits</item>
  <item>probes</item>
  <item>pdb_hpa</item>
  <item>network_policy</item>
  <item>rbac</item>
  <item>secrets</item>
  <item>image_pinning</item>
</checklist>
<output_format>
  <section name="findings"/>
  <section name="fixes"/>
</output_format>
