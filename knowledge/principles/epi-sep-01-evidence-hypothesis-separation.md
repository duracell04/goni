---
id: EPI-SEP-01
title: Evidence-Hypothesis Separation
type: principle
status: draft
implementation_state: not_applicable
proposition: Model-generated hypotheses, inferences, critiques, repetitions, simulations, and confidence signals may guide further cognition but do not constitute independent evidence for the propositions they generate.
domains:
- agent
- memory
- research
- system
aliases:
- EVIDENCE-HYPOTHESIS-SEPARATION
- SYNTHETIC-EVIDENCE-BOUNDARY
relations:
- type: supports
  target: DREAM-01
- type: supports
  target: GONI-SPEC-A742123055E0
sources: []
artifacts: []
uncertainty: This principle defines an epistemic boundary rather than an implementation mechanism. Concrete tests for evidence-lineage independence, source dependence, calibration, and promotion thresholds require implementation and adversarial evaluation.
legacy: []
---

# Evidence-Hypothesis Separation

GONI distinguishes the material used to generate a proposition from evidence that independently supports the proposition as true.

The compact invariant is:

> **Synthetic cognition may guide inquiry. It may not bootstrap itself into independent evidence.**

This applies across interactive reasoning, retrieval-augmented generation, offline cognition, reflection, simulation, model councils, summaries, critiques, and other model-mediated derivations.

## 1. Epistemic classes

A conforming reasoning path MUST preserve the distinction between:

1. **observation or external evidence** — material attributable to an observed event, external source, measurement, user decision, or other qualifying evidence channel;
2. **derivation provenance** — the source material, model/runtime, context, transforms, and parent artifacts used to generate a synthetic proposition;
3. **synthetic cognition** — hypotheses, summaries, critiques, simulations, counterfactuals, confidence signals, or other model-generated derivatives;
4. **confirmation evidence** — qualifying evidence that supports or contradicts the proposition itself independently of the derivation that generated it.

Derivation provenance explains where a hypothesis came from. It does not by itself confirm the hypothesis.

## 2. Independence invariant

For a proposition \(h\), let \(E(h)\) be the set of evidence items presented as support and let \(r(e)\) identify the independent provenance root or evidence-lineage class of item \(e\).

The conceptual distinction is:

\[
N_{raw}(h)=|E(h)|
\]

versus

\[
N_{ind}(h)=|\{r(e): e\in E(h)\}|.
\]

A larger \(N_{raw}\) MUST NOT be interpreted as stronger independent corroboration when the additional items descend from the same underlying evidence lineage.

The concrete function \(r(e)\) is implementation-dependent. This principle does not claim that source independence can always be inferred reliably from metadata alone.

## 3. Synthetic self-corroboration

The following MUST NOT count as independent confirmation merely because they recur:

- repeated generation of the same proposition;
- agreement among outputs derived from the same source material;
- a model critique that restates or endorses an earlier model hypothesis;
- summaries or revisions that descend from the same evidence lineage;
- multiple agents or models whose apparent agreement is conditioned on materially identical evidence;
- simulated or counterfactual outcomes.

Model confidence is a cognitive signal. It is not independent evidence.

## 4. Operational consequence

Synthetic cognition MAY:

- create or reprioritize hypotheses;
- identify contradictions;
- identify evidence gaps;
- propose discriminating tests or retrieval requests;
- change salience or research priority;
- motivate a user-visible uncertainty statement.

It MUST NOT silently promote a proposition to confirmed fact, authoritative preference, policy, mandate, or executable instruction.

Durable promotion remains governed by the applicable MemoryEntries, confirmation, policy, and authority contracts.
