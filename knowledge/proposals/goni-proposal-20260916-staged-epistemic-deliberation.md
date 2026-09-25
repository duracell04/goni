---
id: GONI-PROPOSAL-20260916-STAGED-EPISTEMIC-DELIBERATION
title: Staged epistemic deliberation for consequential multi-agent reasoning
type: proposal
status: draft
implementation_state: specified_only
proposition: For consequential multi-agent reasoning, Goni should evaluate staged independent commitment, blinded critique, blinded meta-review, external verification, provenance-aware aggregation, and explicit preservation of unresolved dissent instead of treating free-form consensus as the default.
domains:
- epistemics
- multi-agent
- orchestration
- audit
aliases:
- STAGED-EPISTEMIC-DELIBERATION
relations:
- type: derived_from
  target: GONI-EVIDENCE-20260916-MAD-CONFORMITY
- type: derived_from
  target: GONI-EVIDENCE-20260916-DIVERSITY-CALIBRATION
- type: derived_from
  target: GONI-EVIDENCE-20260916-EVALUATOR-BIAS
- type: derived_from
  target: GONI-EVIDENCE-20260916-INCENTIVE-AGGREGATION
- type: refines
  target: GONI-SPEC-70C934F7D55A
  note: Extends audit-grade epistemic discipline to multi-agent reasoning and review.
- type: refines
  target: GONI-SYNTHESIS-312D277D0C2E
  note: Makes the Pia/provenance layer more precise for multi-agent claims and reviews.
sources:
- SRC-GONI20260916-EPISTEMIC-DELIBERATION-NOTE
artifacts: []
uncertainty: This is a design proposal, not an implemented or validated runtime protocol. The optimal number of agents, review assignments, blinding level, model heterogeneity, stopping rule, and verification budget remain empirical questions.
legacy: []
---

# Staged epistemic deliberation for consequential multi-agent reasoning

> Status boundary: this is a design proposal. It does not assert that Goni currently implements or validates the protocol.

The proposed default for high-consequence multi-agent reasoning is a staged information-flow protocol:

1. **Independent commitment.** Solvers receive the same task contract but cannot inspect one another's outputs. Each commits claims, assumptions, uncertainty, evidence references, and falsification conditions.
2. **Blinded independent critique.** Reviewers inspect frozen candidate solutions in randomized or otherwise controlled order. They cannot see peer reviews while writing their own.
3. **Blinded meta-review.** Auditors test whether specific criticisms are logically and evidentially warranted. Auditors do not see one another's judgments before commitment.
4. **External verification.** Whenever the task admits deterministic or empirical checks, the system moves from model opinion to tests, primary sources, computation, proof checking, execution, or observed outcomes.
5. **Provenance-aware aggregation.** The synthesizer aggregates claims and evidence lineages rather than treating raw vote count as independent support.
6. **Dissent preservation.** Material unresolved alternatives remain visible with the evidence or observation that would discriminate among them.

The architecture intentionally delays coupling. Interaction is introduced only after initial hypotheses are frozen, and each later layer has a distinct epistemic function.

The proposal does **not** prescribe universal anonymity. Full provenance remains in the system record. Identity and lineage information are selectively hidden from reviewers when they would create evaluation bias, then made available when they are relevant to calibration, conflict-of-interest, or failure-mode analysis.
