---
name: master
description: Run the full command pipeline — plan, swarm, build, verify, ship
---

<role>production_pipeline_orchestrator</role>
<objective>Take {{parameters}} from raw prompt to production by executing every available command in its correct position — planning before code, review before hardening, verification before release.</objective>
<input><prompt>{{parameters}}</prompt></input>
<pipeline>
  <stage id="1" name="understand" uses="explain">Restate the goal, surface unknowns, list what must not change.</stage>
  <stage id="2" name="design" uses="tpl-design">Produce the design/plan artifact; identify affected surfaces and blast radius.</stage>
  <stage id="3" name="decompose" uses="swarm">Split into independent workstreams with bounded files, constraints, and a verify command per specialist.</stage>
  <stage id="4" name="implement" uses="tpl-implement">Execute the plan smallest-change-first; TDD where a test seam exists (tpl-tdd).</stage>
  <stage id="5" name="review" uses="review">Self-review the diff against repo conventions before any external reviewer runs.</stage>
  <stage id="6" name="harden" uses="harden">Apply the security and robustness pass; threat-model any new input boundary (threat-model).</stage>
  <stage id="7" name="verify" uses="test-plan">Run the stack's verification matrix (fmt, clippy -D warnings, tests) and prove the changed flow works.</stage>
  <stage id="8" name="release" uses="ship">Commit with a conventional scoped message, open the PR, state rollout and rollback.</stage>
</pipeline>
<rules>
  <rule>Never skip a stage; if a stage is genuinely not applicable, say why in one line and move on.</rule>
  <rule>Stages 1–3 produce no code changes.</rule>
  <rule>Each stage's output feeds the next stage as input context.</rule>
  <rule>On any stage failure, stop the pipeline, report the failing stage, and propose the fix — do not silently continue.</rule>
</rules>
<output_format>
  <section name="pipeline_log" desc="one line per completed stage with its verdict"/>
  <section name="artifacts" desc="plans, diffs, PR links"/>
  <section name="verification_evidence" desc="actual command outputs, not claims"/>
</output_format>
