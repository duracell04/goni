---
id: GONI-IMAP-66D2DBD94204
title: Adaptive working-set management
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Goni should adapt retrieval breadth, context budget, residency tier, and cache admission or eviction to measured relevance concentration, task state, consequence, and reconstruction cost while preserving the existing Context Plane and Memory Plane contracts.
domains:
- data
- kernel
- software
- system
aliases:
- semantic-working-set-management
relations:
- type: depends_on
  target: GONI-PRINCIPLE-9062425CD490
- type: refines
  target: GONI-IMAP-086FADF076BA
- type: refines
  target: GONI-IMAP-E2D8A099B27A
- type: refines
  target: GONI-SPEC-2F762878317A
sources: []
artifacts: []
uncertainty: This is a specified architecture extension. Thresholds, concentration metrics, residency tiers, and cache policies require empirical calibration and must not be presented as implemented behavior.
legacy: []
---

# Adaptive working-set management

Goni's existing retrieval and Context Plane already separate a large durable
state from a bounded model context. Adaptive working-set management makes the
resource boundary itself responsive to the current task.

## 1. Adaptive candidate breadth

Candidate generation SHOULD begin from hardware-friendly discrete retrieval
budgets such as:

[
K in {128, 256, 512, 1024},
]

or backend-specific equivalents. The selected bucket may depend on relevance
concentration, query complexity, risk, and available latency or token budgets.

A conceptual sufficient-mass rule is:

[
K(q)=minleft{K:
rac{sum_{r=1}^{K}s_r}
{sum_{r=1}^{N}s_r}ge	au
ight}.
]

The implementation is not required to fit a power law. Entropy, cumulative
mass, score gaps, calibrated recall estimates, or another validated
concentration statistic may control the bucket.

Candidate generation SHOULD bias toward recall. Missing one decisive item can
be more damaging than retrieving several irrelevant candidates, while later
reranking and submodular selection can remove false positives.

## 2. Context budget remains a separate decision

Retrieval breadth (K) and final context budget (B) are independent controls.

[
	ext{retrieve candidates}
ightarrow
	ext{rerank/filter}
ightarrow
	ext{submodular selection under }B.
]

A broad candidate set need not imply a large prompt. This preserves the existing
Context Plane guarantee while allowing Goni to search more widely when evidence
is diffuse.

## 3. Residency tiers

Memory lifecycle classes remain semantic categories. Residency is an orthogonal
systems property:

- **hot** — current task or project state at highest useful fidelity;
- **warm** — high-probability reusable state, summaries, embeddings, or
  compressed representations;
- **cold** — canonical durable state that remains addressable but is not kept in
  the active working set;
- **external** — policy-permitted connectors or remote sources accessed only
  through existing mediation.

Promotion and demotion SHOULD consider recency, reuse frequency, task or project
affinity, consequence, reconstruction cost, and storage or bandwidth cost.

## 4. Utility-aware admission and eviction

The baseline policy SHOULD compare recency-only strategies with an explicit
utility model. A conceptual score is:

[
Score_i=
rac{
P(mathrm{reuse}_i)
	imes
Impact_i
	imes
ReconstructionCost_i
}{
StorageCost_i
}.
]

This expression is a research scaffold, not a fixed production formula.
Semantic importance and popularity MUST remain separable: rarely accessed state
may still be decisive.

Admission MAY be predictive. At creation time, Goni may estimate:

[
P(	ext{future relevance}mid
	ext{content, task, project, position, access history}).
]

## 5. Temporal locality

The active working set SHOULD exploit continuity across adjacent work. Consecutive
requests in the same task or project often reuse overlapping state, so a
project-aware working set can reduce repeated retrieval, prefill, and
reconstruction work.

Reuse MUST remain invalidatable when policy, mandate, provenance, source state,
or user intent materially changes.

## 6. Observability

A future implementation SHOULD log enough information to evaluate:

- retrieval bucket selected;
- concentration statistic;
- candidate recall and downstream citation coverage;
- context tokens selected;
- cache hit rate;
- promotion/demotion decisions;
- estimated versus observed reuse;
- latency, bandwidth, memory, energy, and quality deltas.

The purpose is closed-loop allocation: measure whether a smaller active set
preserves the required answer and action quality before making the policy more
aggressive.
