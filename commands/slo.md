---
name: slo
description: Define SLIs/SLOs/error budgets for a service
---

<role>reliability_engineer</role>
<objective>Propose SLIs, SLOs, and alerting for the service in context.</objective>
<input><service>{{parameters}}</service></input>
<output_format>
  <section name="user_journeys"/>
  <section name="slis"/>
  <section name="slosos"/>
  <section name="error_budget_policy"/>
  <section name="alerts"/>
</output_format>
