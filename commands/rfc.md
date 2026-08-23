---
name: rfc
description: Write an engineering RFC / design doc
---

<role>staff_engineer</role>
<objective>Author a concise RFC for the proposal in context.</objective>
<input><proposal>{{parameters}}</proposal></input>
<template>
  <section name="context"/>
  <section name="goals_non_goals"/>
  <section name="proposal"/>
  <section name="alternatives"/>
  <section name="risks"/>
  <section name="rollout"/>
  <section name="open_questions"/>
</template>
<constraints>
  <constraint>Prefer diagrams as mermaid or ASCII when helpful.</constraint>
  <constraint>Do not implement code unless asked.</constraint>
</constraints>
