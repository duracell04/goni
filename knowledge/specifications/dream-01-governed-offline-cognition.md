---
id: DREAM-01
title: DREAM-01 - Governed Offline Cognition
type: specification
status: draft
implementation_state: specified_only
proposition: Kernel-scheduled offline cognition may generate synthetic cognitive candidates from authorized context, but those candidates must preserve explicit epistemic status and may not by themselves become facts, executable instructions, or authority.
domains:
- agent
- kernel
- memory
- research
- specs
- system
aliases:
- GOVERNED-DREAMING
- OFFLINE-COGNITION
relations:
- type: depends_on
  target: GONI-DECISION-6D3A2B71C9E4
- type: depends_on
  target: GONI-DECISION-FEB39440C565
- type: depends_on
  target: GONI-DECISION-9AF49466170D
- type: depends_on
  target: SCHED-01
- type: depends_on
  target: GONI-SPEC-A742123055E0
- type: depends_on
  target: EPI-SEP-01
- type: refines
  target: GONI-SPEC-33294E0D3306
sources:
- SRC-HA2018-WORLD-MODELS
- SRC-HAFNER2023-DREAMERV3
- SRC-PARK2023-GENERATIVE-AGENTS
- SRC-VANDEVEN2020-BRAIN-REPLAY
- SRC-OPENAI2026-DREAMING
artifacts: []
uncertainty: DREAM-01 specifies a proposed boundary, not an implemented or empirically validated subsystem. Scheduling value, hypothesis quality, contamination resistance, compute efficiency, and promotion thresholds require matched evaluation.
legacy: []
---

# DREAM-01 - Governed Offline Cognition

> Status boundary: this is a specified-only contract. Enforcement language
> describes intended conformance behavior rather than observed implementation.

## 1. Scope

DREAM-01 defines bounded cognition that runs outside an immediate
latency-critical user turn.

A dream cycle is not a new architectural plane, a new memory database, or a new
source of authority. It is a kernel-scheduled cognitive protocol over the
existing Data, Context, Control, and Execution responsibilities.

The compact invariant is:

> **Dreaming may expand the hypothesis space. It may not expand the authority space.**

## 2. Inputs

A dream cycle MUST receive information through the same governed memory and
context boundaries as interactive cognition.

The cycle MAY operate over:

- episodic observations;
- semantic and project memory;
- prior derived or hypothesis entries;
- receipts and outcomes;
- contradiction or staleness signals;
- current goals or Work Order context when policy permits.

The cycle MUST preserve source and parent references sufficient to distinguish
observed evidence from model-generated derivatives.

## 3. Permitted cognitive operations

A dream cycle MAY perform:

- replay;
- consolidation;
- abstraction;
- association;
- contradiction detection;
- counterfactual generation;
- future-state simulation;
- alternative explanation generation;
- falsification attempts;
- hypothesis generation;
- evidence-gap identification;
- discriminating-test generation.

These are cognitive operations. They do not carry execution authority.

## 4. Output classes and persistence

A dream cycle MAY produce candidates such as:

- association;
- abstraction;
- causal_hypothesis;
- counterfactual;
- simulation;
- contradiction;
- opportunity;
- evidence_request;
- test_candidate.

Durable dream outputs MUST use the existing governed memory substrate.
MemoryEntries remains canonical in v1.

DREAM-01 MUST NOT create a new fact, user decision, authoritative preference,
policy, mandate, capability, approval, or executable procedural instruction
merely because a model generated the content.

New synthetic durable entries MUST remain hypothesis or derived until a
separate evidence-backed transition satisfies the applicable memory contract.
A deterministic compaction or summary of existing facts remains a derived
representation of those facts rather than a new independent fact.

No DreamStore or parallel hypothesis database is introduced by this contract.

## 5. Synthetic provenance and epistemic quarantine

For a durable dream result, existing value/provenance fields SHOULD preserve,
where available:

- origin_mode = dream;
- dream_run_id;
- candidate_type;
- parent memory or entity references;
- source evidence references;
- model bundle identifier;
- inference-frame or context-pack identifier;
- receipt reference;
- generation timestamp;
- review or expiry state.

Until a future schema revision demonstrates the need for first-class columns,
these fields may remain governed values inside the existing memory record.

DREAM-01 applies the general evidence-hypothesis separation invariant in
EPI-SEP-01. Dream-specific rules additionally require:

1. A simulated or counterfactual event is not an observed event.
2. Promotion to a confirmed fact requires evidence that supports the derived
   proposition itself under the confirmed-vs-speculation contract.

These rules prevent recursive synthetic-memory contamination.

## 6. Authority boundary

Dream output MUST NOT create, widen, or refresh authority.

The governing transition rule is:

> **A change in dream state does not imply a change in authority state.**

A dream cycle may produce a proposal or candidate Work Order. Consequential
execution still requires the ordinary Goni path through canonical mandate,
policy, capability, approval, budget, revocation, and tool mediation.

A model-visible authority summary may assist cognition but remains subordinate
to canonical kernel-owned authority state.

## 7. Scheduling and resource control

Dream work MUST be submitted through the existing scheduler.

The default scheduling posture SHOULD treat dream work as background,
preemptible, and resource-bounded relative to interactive work.

A dream job SHOULD declare:

- input scope;
- context/token budget;
- solver/model budget;
- maximum recursion or branch depth where applicable;
- cancellation semantics;
- persistence policy;
- allowed completion states.

Idle compute is an optimization opportunity, not an entitlement to unbounded
background inference.

## 8. Network and external effects

Local-first policy remains authoritative during dreaming.

Remote inference, connector access, or other egress may occur only through the
existing governed egress path and only when policy explicitly permits it.

A dream cycle has no ambient tool authority. External observation or action
needed to test a hypothesis must enter the appropriate governed retrieval,
connector, or Work Order path.

## 9. Receipts and reviewability

Consequential durable writes from a dream cycle SHOULD remain reconstructable
through existing receipt and provenance mechanisms.

A reviewer should be able to determine:

- which observations and memories were used;
- which outputs were synthetic;
- which model/runtime produced them;
- which evidence, if any, later supported or refuted them;
- whether a result remained unresolved, expired, was quarantined, or was
  promoted through a separate evidence-backed transition.

## 10. Failure handling

Contradictory, weakly grounded, stale, poisoned, or otherwise unsafe dream
outputs MAY use the existing quarantine policy alias and normal review/TTL
semantics.

A failed dream cycle should degrade to unresolved candidates or no durable
write. Failure must not be converted into broader authority or fabricated
certainty.

## 11. Conformance invariants

A conforming implementation must preserve at least these invariants:

- synthetic-to-fact promotion without qualifying evidence: zero;
- authority expansion caused solely by dream output: zero;
- model self-corroboration counted as independent evidence: zero;
- dream output bypassing ordinary tool/effect mediation: zero.

Utility, recall, calibration, compute cost, and scheduling quality are measured
properties rather than assumed guarantees.
