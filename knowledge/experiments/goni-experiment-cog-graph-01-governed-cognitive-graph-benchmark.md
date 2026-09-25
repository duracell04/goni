---
id: GONI-EXPERIMENT-COG-GRAPH-01
title: Governed Cognitive Graph Benchmark
type: experiment
status: draft
implementation_state: not_applicable
proposition: "Goni should compare monolithic frontier-model execution, simple small-to-large escalation, fixed specialist cascades, dynamic heterogeneous cognitive graphs, and multi-agent/multi-model graphs under matched Work Orders, tools, authority, and evaluation budgets before promoting cognitive-graph complexity."
domains:
- agent
- evaluation
- models
- routing
aliases: []
relations:
- type: tests
  target: COG-GRAPH-01
- type: tests
  target: AGENT-MODEL-ORTHO-01
- type: tests
  target: MODEL-ROLE-01
- type: tests
  target: ROUTE-SIGNAL-01
- type: tests
  target: POLICY-PROJECTION-01
- type: tests
  target: GONI-PRINCIPLE-TRAJ-ECON-01
sources:
- SRC-LIU2026-VLLM-SEMANTIC-ROUTER
- SRC-WANG2024-MIXTURE-OF-AGENTS
- SRC-CHEN2023-AGENTVERSE
- SRC-WANG2025-WHEN-TO-REASON
artifacts: []
uncertainty: "This is a planned matched evaluation. More complex graphs may improve quality on some workloads while losing on latency, cost, correlated error, integration burden, or verification overhead."
legacy: []
---

# Governed Cognitive Graph Benchmark

## Research question

Does dynamically composing heterogeneous cognitive mechanisms improve verified
task completion enough to justify its additional routing, orchestration,
verification, and integration cost?

## Compared architectures

At minimum compare:

A. Monolithic frontier model
- one strong general model receives the task and available tools.

B. Simple escalation
- cheaper/default model first;
- stronger model only when the configured escalation condition fires.

C. Fixed specialist cascade
- a predetermined chain such as classify -> generate -> verify.

D. Dynamic heterogeneous cognitive graph
- runtime route selection among deterministic code, retrieval, classifiers,
  decision models, generators, reasoners, verifiers, and other approved roles.

E. Multi-agent / multi-model graph
- fan-out to multiple specialist or independent agents/models followed by
  governed synthesis or verification.

## Matched conditions

The comparison SHOULD hold constant:

- Work Order and Done Contract;
- available source material;
- tool capabilities;
- policy and authority corridor;
- network eligibility;
- final output requirements;
- task dataset;
- evaluation oracle or human rubric;
- retry and maximum wall-clock envelope where practical.

Architecture-specific compute budgets must be measured rather than hidden.

## Primary outcome

The primary system metric is verified successful delegated work, not raw model
benchmark score.

Useful normalized measures include:

verified task success / monetary cost

verified task success / wall-clock time

verified task success / energy where measurable.

## Cognitive metrics

Report:

- task success and semantic quality;
- frontier-model invocation rate;
- graph node count actually executed;
- trajectory length;
- escalation rate;
- false escalation rate;
- late escalation rate;
- abstention rate;
- verification catch rate;
- verifier false-accept and false-reject rates where labels exist;
- calibration and risk-coverage for confidence-bearing decisions;
- repair-loop count and stop reason.

## System metrics

Report:

- p50/p95 end-to-end latency;
- monetary inference/tool cost;
- input/output token volume;
- context reconstruction/compression cost;
- local energy or thermal proxy where measurable;
- network/egress volume;
- privacy-class exposure;
- human interruption minutes;
- receipt completeness;
- policy-denied route attempts;
- unauthorized-effect incidents, which should remain zero under the kernel
  boundary.

## Complexity accounting

Graph complexity itself is a cost.

Track:

- number of mechanism/provider dependencies;
- configuration surface;
- policy-projection count;
- orchestration failures;
- correlated-error cases;
- integration/verification overhead.

A dynamic graph is preferred only when its marginal benefit exceeds these costs.

## Agent-model orthogonality check

For architectures C through E, replay equivalent tasks while changing eligible
model bundles for the same cognitive roles.

The agent's identity, mandate, memory ownership, tools, and authority should
remain unchanged when the model substrate changes.

## Falsification

The cognitive-graph architecture is weakened for a target workload if a simpler
architecture achieves equal or better verified task success with lower total
trajectory cost, latency, privacy exposure, and operational complexity.

The experiment therefore allows the result that one strong model is preferable
for a given workload.
