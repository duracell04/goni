---
id: GONI-SPEC-A742123055E0
title: 5.1 Confirmed vs speculation thresholds
type: specification
status: draft
implementation_state: specified_only
proposition: 'For MemoryEntries, confirmation requires qualifying evidence for the proposition itself; synthetic derivation provenance, model confidence, and repeated model generation do not independently confirm a derived claim.'
domains:
- specs
aliases: []
relations:
- type: refines
  target: DREAM-01
sources: []
artifacts: []
uncertainty: The legacy threshold is retained for direct evidence-backed claims but narrowed for synthetic derivatives. Concrete independence tests for evidence lineages require implementation and adversarial evaluation.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 5.1 Confirmed vs speculation thresholds
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 5.1 Confirmed vs speculation thresholds

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5.1 Confirmed vs speculation thresholds

For MemoryEntries:
- A direct claim may be **confirmed** if `confirmed_by_event_id` is present, or
  if `source_chunk_ids` contains qualifying evidence for the proposition,
  `confidence` meets the policy threshold, and `conflict_state` is not
  contradictory.
- Otherwise, the claim MUST be stored as `hypothesis` or `derived` with a
  `ttl_ms` or `review_at` value, and MUST NOT be promoted to `fact` without
  new evidence.
- For `hypothesis` or `derived` entries produced by dreaming, simulation,
  counterfactual reasoning, reflection, or other synthetic inference,
  `source_chunk_ids` establish derivation provenance only. They do not by
  themselves confirm the newly derived proposition.
- Model confidence, repeated generation of the same proposition, or agreement
  among model-generated descendants of the same evidence lineage MUST NOT count
  as independent confirmation.
- Promotion of a synthetic derivative to `fact` requires qualifying evidence
  that supports the derived proposition itself and remains distinguishable from
  the model-generation lineage that produced it.
- Simulated or counterfactual events MUST NOT populate
  `confirmed_by_event_id` as though they were observed external events.

This preserves an explicit boundary between:

1. **evidence provenance** — what material a cognition process used;
2. **derivation provenance** — how a synthetic proposition was produced; and
3. **confirmation evidence** — what independently supports the proposition as
   true.

Under `DREAM-01`, repeated synthesis may increase salience or motivate testing,
but it cannot bootstrap a hypothesis into a fact through self-corroboration.
