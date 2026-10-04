---
id: GONI-DECISION-0532C27DA3CC
title: D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter
type: decision
status: draft
implementation_state: specified_only
proposition: All schedulable work enters the kernel queueing network, while compute-bearing jobs expose enough contract metadata for the scheduler to jointly choose placement, execution path, and acceptable evidence path without bypassing GONI authority.
domains:
- software
- compute
- scheduler
aliases: []
relations:
- type: depends_on
  target: COMP-01
- type: depends_on
  target: EXEC-01
- type: depends_on
  target: EVID-01
- type: refined_by
  target: GONI-PRINCIPLE-QUALIFIED-PROVIDER-SUBSTITUTABILITY
sources:
- SRC-RAY-SCHEDULING-LOCALITY
- SRC-ROOFLINE-CACM
artifacts: []
uncertainty: The exact MaxWeight objective, queue classes, estimator calibration, and multi-node scheduling policy require experiment-backed tuning.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-008 – Lyapunov / MaxWeight scheduler as the unique work arbiter

> Status boundary: this remains a specified-only scheduling architecture. No claim is made that the full multi-resource or evidence-aware scheduler is implemented.

## Formal statement

All work units, including model calls, embeddings, indexing, compaction, tool-support computation, and future external compute contracts, are represented as jobs in the queueing network `K`. No component should maintain a hidden unbounded work queue outside the kernel's scheduling boundary.

For compute-bearing jobs, the scheduler should consume the relevant COMP-01, EXEC-01, and EVID-01 metadata and jointly select:

1. placement;
2. execution path;
3. evidence path.

The scheduler remains subordinate to kernel authority. A cheaper or faster placement is not eligible when it violates capability, privacy, sovereignty, evidence, or policy constraints.

## Cost model

Candidate routes should be compared using an explicit total-cost model rather than execution time alone:

[
C_{total} =
C_{execution}
+ C_{evidence}
+ C_{verification}
+ C_{transfer/storage}
+ C_{coordination/settlement}
+ E[C_{failure}]
]

Latency, energy, privacy exposure, or other objectives may be optimized separately where they are not reducible to a common monetary unit.

The scheduler should model locality and communication costs. A faster accelerator can lose end-to-end if moving model state, input data, intermediate tensors, or proof artifacts dominates the saved compute time.

## Pre-execution versus post-execution facts

The scheduler can reason before execution about whether an evidence path is feasible and what it is expected to cost. It cannot generally assume `Verify(result) = true` before the result and evidence exist. Verification is an acceptance step after evidence production.

## Consequences

- Runtime engines and indexers expose backpressure and capability information.
- Compute providers advertise capabilities rather than application identities.
- Job descriptors can reference computation and evidence contracts without replacing delegation-level policy.
- Adding a long-running work class requires updating `K` and its invariants rather than creating an ad-hoc queue.
