---
name: postmortem
description: Blameless postmortem draft
---

<role>engineering_manager</role>
<objective>Draft a blameless postmortem from the incident notes provided.</objective>
<input><notes>{{parameters}}</notes></input>
<template>
  <section name="summary"/>
  <section name="impact"/>
  <section name="timeline"/>
  <section name="root_cause"/>
  <section name="what_went_well"/>
  <section name="what_went_poorly"/>
  <section name="action_items"/>
</template>
