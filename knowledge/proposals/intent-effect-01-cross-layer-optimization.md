---
id: INTENT-EFFECT-01
title: 'Intent-to-Effect Optimization as a Cross-Layer Research Hypothesis'
type: proposal
status: draft
implementation_state: specified_only
proposition: 'GONI should evaluate whether coordinating context, authorization, inference placement, scheduling, and execution reduces total cost per correct, authorized, externally verified task relative to independently optimized components while retaining existing trust boundaries.'
domains:
- agent
- hardware
- research
- system
aliases: []
relations:
- type: refines
  target: GONI-DECISION-1DF4EF699554
- type: depends_on
  target: GONI-DECISION-42DF477FE7FB
- type: depends_on
  target: INF-PLACE-01
- type: depends_on
  target: TOOL-01
- type: depends_on
  target: AGENT-NET-VALUE-01
sources:
- SRC-MEI2024-AIOS
- SRC-HORVITZ1999-MIXED-INITIATIVE
- SRC-XIE2024-OSWORLD
artifacts: []
uncertainty: 'No GONI measurements establish cross-layer gains. Coordination may increase overhead and technical debt; external sources motivate research but do not prove superiority.'
legacy: []
---

# Intent-to-Effect Optimization

## Research question

For a defined distribution of delegated tasks, does coordination across GONI's existing A (data), X (context), K (control), and E (execution) planes lower total cost of correctly authorized and externally verified outcomes compared with independently optimized components?

This proposal defines an evaluation objective, not a new kernel or mandatory orchestration subsystem. Existing operating-system protection, capability boundaries, and four-plane architecture remain intact.

## Outcomes and constraints

Measure task latency from accepted request to verified external effect:

\[
T_{\mathrm{intent\rightarrow effect}}=t_{\mathrm{verified\ effect}}-t_{\mathrm{accepted\ request}}.
\]

Count denials, failed tasks, timeouts, and unverified outcomes explicitly rather than excluding them from denominators. Distinguish policy approval, tool acknowledgment, and external-state verification.

A candidate aggregate efficiency metric is total system cost divided by the number of correct, authorized, verified tasks, when that count is nonzero. Report correctness, authorization, tail latency, human intervention, resource usage, and failure costs separately. Total cost includes compute, energy, external services, verification, retries, and human time; longer-term integration and maintenance costs require explicit estimates.

Safety, user authority, required evidence, and network policies remain constraints on optimization. Work stops when the DoneContract is satisfied; optional elaboration follows AGENT-NET-VALUE-01.

## Candidate mechanisms

Potential savings may arise from reducing repeated interpretation and model calls, reusing task-relevant context, selecting typed APIs where available, hardware-aware inference admission, and avoiding unnecessary subtasks. Their benefits remain hypotheses.

Use critical-path analysis: if a short stage is optimized while external I/O, human review, or model inference dominates, the net improvement may be negligible. Cross-layer coordination also introduces its own overhead.

## Evaluation boundary

EXP-INTENT-EFFECT-01 specifies the falsification protocol. Related literature on AI agent operating systems and mixed-initiative interaction supplies prior art, not evidence of GONI-specific improvement. Promote changes to existing architectural decisions only after reproducible, pinned experimental evidence.
