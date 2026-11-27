---
id: EXP-COG-JOINT-01
title: Evaluation of heterogeneous cognitive joints
type: experiment
status: draft
implementation_state: not_applicable
proposition: A matched experiment should test whether process-first heterogeneous cognitive graphs attain better
  end-to-end accepted-task economics and latency than a conventional LLM harness and an LLM-only modular graph.
domains:
- research
- evaluation
- agent
- hardware
- software
aliases: []
relations:
- type: tests
  target: COG-JOINT-01
  note: Compare modularity and decision mechanisms at equivalent operational boundaries.
- type: tests
  target: COG-SPEC-01
  note: Measure cost/quality frontier and specialization break-even.
- type: tests
  target: OBJ-COG-JOINT-01
  note: Probe stale-state, correlated-error and coordination failures.
- type: depends_on
  target: EXP-INF-PLACE-01
  note: Associate measurements with named hardware/runtime profiles.
- type: depends_on
  target: EVID-HARNESS-01
  note: Use governed evaluation traces and versioned harness changes.
sources:
- SRC-DEUSSER2026-JEV
- SRC-YANG2026-JEV-LIMITS
- SRC-IMPERATRICE2026-JEV-BIAS
- SRC-GEIFMAN2017-SELECTIVE
artifacts: []
uncertainty: Protocol only. No experiment has been executed; quality thresholds, workload prevalence, resource
  limits, sample sizes, and hardware/provider choices require prespecification.
legacy: []
---

# Evaluation of heterogeneous cognitive joints

## Comparison

Use bounded ordinary document triage, inbox categorization, routing,
field extraction and reconciliation advisory tasks with independently
labeled expected results. Choose tasks from authorized, sanitized or synthetic
data; replay without external effects. Keep the same work order, reference
state, allowed actions, explicit authority checks, validation, maximum
budget and completion criteria.

- A: straightforward LLM plus conventional harness and deterministic tools.
- B: explicit workflow with distinct generative-model modules.
- C: identical workflow contracts with heterogeneous candidates such as
  deterministic rules, a local classifier or small model, learned ranker,
  optional world model for a demonstrably predictive task, and budgeted
  deliberative escalation. Treat Jev as a separately reported, opt-in hosted
  comparator with permissible data. It cannot establish an offline result.
- D: best applicable all-deterministic or conventional ML baseline to test
  whether any generative or agentic machinery is justified.

Separate the graph-topology effect (A vs B) from model-mechanism substitution
(B vs C). Pair cases across arms, freeze task splits and versions, use
blind correctness review, and record ablations of relevant joints. Compare
local-only and approved optional remote profiles separately.

## Measurements

Prespecify task success and error severity, acceptable false-positive/negative
rates, critical failure classes, maximum latency, budget and review burden,
statistical reporting and sample-size logic before observing the evaluation
split. Report end-to-end accepted completion conditional on authority
compliance and cost per accepted task; also report coverage/abstention,
calibration (Brier score, reliability by cohort), escalation, rework,
p50/p95 latency, throughput, resource utilization, joules per accepted task
where instrumented, input/output token counts, error cascades, dependencies,
maintenance time and robustness under version changes.

Probe multiple updates to one entity, stale context, missing evidence,
out-of-domain inputs, option order, conflicting inputs, timeouts and
backend failure. Test that proposals cannot become authorized effects
when stale, invalid, or unpermitted. Count error and review cost as
ordinary economic outcomes rather than omit failed calls. Retain
negative results and measured-versus-estimated distinctions.

Record exact model/runtime identities, version and configuration, hardware,
data/evaluation manifests, pricing timestamp and relevant terms. Measure
the development/maintenance cost and repeated-use volume that would justify
each replacement; verify cross-layer deployment feasibility independently.

## Acceptance boundary

The hypothesis is weakened if the simpler baseline matches or exceeds
task-level economics at the agreed risk/quality/latency level, if
error propagation undermines acceptance, or if maintenance and deployment
costs exceed the observed savings. No relaxation of the authority boundary
is admissible as a performance gain.

A future runnable experiment belongs in the prototype lab or separately
approved experiment repository. Only repository-and-full-commit-pinned
results with stated scope, including failed results, constitute Goni
implementation/measurement evidence. This blueprint adds no runtime code.
