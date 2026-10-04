---
id: GONI-EXPERIMENT-DECISION-LAYER-01
title: Heterogeneous Decision-Layer Benchmark
type: experiment
status: draft
implementation_state: not_applicable
proposition: "Goni should compare deterministic rules, semantic routers, task-specific classifiers, bounded decision models, small structured-output generative models, and stronger reasoning models on matched bounded-decision workloads before promoting any mechanism as a default route."
domains:
- evaluation
- models
- routing
aliases: []
relations:
- type: tests
  target: GONI-PRINCIPLE-HET-INTEL-01
- type: tests
  target: DECISION-MODEL-01
- type: tests
  target: CAL-01
- type: tests
  target: SELECTIVE-01
- type: tests
  target: LOSS-01
- type: tests
  target: GONI-IMAP-EB2133E6965D
sources:
- SRC-FRUGALGPT2023
- SRC-ROUTELLM2024
- SRC-ROUTERBENCH2024
artifacts: []
uncertainty: "This is a planned matched evaluation. No mechanism is assumed to dominate globally; results are expected to vary by workload, hardware, consequence class, and deployment constraints."
legacy: []
---

# Heterogeneous Decision-Layer Benchmark

## Research question

For bounded semantic decisions, where does each computational primitive lie on
the quality-cost-latency-calibration-privacy frontier under matched conditions?

## Candidate mechanisms

At minimum, compare:

1. deterministic rule or exact program where a meaningful rule baseline exists;
2. embedding or semantic router;
3. task-specific classifier;
4. generalized bounded semantic decision model;
5. small generative model with constrained structured output;
6. stronger deliberative/reasoning model.

A benchmark MAY omit an inapplicable mechanism for a task, but the omission and
reason MUST be recorded.

## Matched conditions

Comparisons SHOULD hold constant as far as possible:

- task definition and label ontology;
- train/calibration/test split;
- available task context;
- tool permissions;
- authority corridor;
- target output schema;
- hardware or remote-provider accounting;
- retry policy;
- evaluation budget.

Calibration data MUST remain distinct from final held-out evaluation data.

## Metrics

Report task-appropriate semantic quality, such as accuracy, macro/micro F1, or
task-specific loss.

For confidence-bearing mechanisms report where applicable:

- Brier or other proper score;
- negative log-likelihood;
- ECE with binning definition;
- reliability diagram;
- risk-coverage curve;
- area-under-risk-coverage or another declared summary.

Operational metrics SHOULD include:

- correct local accept rate;
- false local accept rate;
- unnecessary escalation rate;
- late escalation rate;
- false-authorize attempts before kernel policy;
- p50/p95 latency;
- monetary cost;
- token/input-output volume where applicable;
- local energy or thermal cost where measurable;
- network/egress exposure;
- trajectory-level cost for multi-step workflows.

## Consequence sensitivity

Metrics MUST be stratified by consequence class where the loss structure differs.
A mechanism that is adequate for reversible classification is not thereby
validated for high-impact autonomous action.

## Promotion rule

No mechanism becomes a default solely from vendor benchmarks, architecture
claims, or aggregate accuracy.

Promotion requires representative matched evidence showing that it occupies an
acceptable operating region for the intended task and consequence class while
preserving privacy, policy, and authority invariants.

## Falsification

The heterogeneous-intelligence thesis is weakened for a target workload if a
single mechanism consistently dominates realistic alternatives across semantic
quality, calibrated risk, latency, economic cost, privacy exposure, and
verification burden under matched conditions.
