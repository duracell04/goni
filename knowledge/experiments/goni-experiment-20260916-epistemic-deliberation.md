---
id: GONI-EXPERIMENT-20260916-EPISTEMIC-DELIBERATION
title: Evaluate staged epistemic deliberation against debate and independent baselines
type: experiment
status: draft
implementation_state: specified_only
proposition: A matched-budget experiment should test whether staged epistemic deliberation improves reliability, calibration, and adversarial robustness relative to free-form debate, independent ensembling, self-consistency, and single-model baselines.
domains:
- epistemics
- multi-agent
- evaluation
aliases: []
relations:
- type: tests
  target: GONI-PROPOSAL-20260916-STAGED-EPISTEMIC-DELIBERATION
sources:
- SRC-KRAIDIA2026-ADVERSARIAL-MAD
- SRC-CUI2026-FREE-MAD
- SRC-SMIT2024-MAD
- SRC-ZHU2026-MAD-DIVERSITY
- SRC-BAND2024-LINGUISTIC-CALIBRATION
artifacts: []
uncertainty: No result is claimed. The experiment must control model family, token/latency budget, task composition, retrieval/tool access, and adversarial conditions before comparing protocols.
legacy: []
---

# Evaluate staged epistemic deliberation against debate and independent baselines

## Research question

At a matched inference budget, does staged epistemic deliberation improve epistemic performance relative to simpler baselines?

## Baselines

At minimum:

- single-agent reasoning;
- self-consistency or repeated independent sampling;
- independent multi-model ensemble with no discussion;
- conventional free-form multi-agent debate;
- consensus-free or anti-conformity debate where available;
- staged independent commitment, critique, meta-review, verification, and aggregation.

## Experimental factors

Vary independently where feasible:

- homogeneous versus heterogeneous model families;
- shared versus distinct retrieval sources;
- clean versus persuasive-adversary conditions;
- reviewer identity visibility versus blinding;
- randomized versus fixed candidate order;
- one review layer versus one review plus one meta-review;
- LLM-only verification versus external deterministic/primary-source verification.

## Metrics

Use task-appropriate outcome metrics together with:

- final-task accuracy or utility;
- Brier or logarithmic score when probabilistic outcomes are observable;
- calibration error;
- false-consensus rate;
- adversarial robustness;
- reviewer error-detection precision and recall where labels exist;
- disagreement retention when evidence remains unresolved;
- number and diversity of independent evidence lineages;
- token cost, wall-clock latency, and marginal gain per additional stage.

Conditional mutual information is useful as a conceptual criterion for whether a review adds information beyond a solution, but it is generally latent in open-ended tasks. Operational evaluations should therefore use measurable proxies rather than claiming direct mutual-information estimates without an appropriate probabilistic model.

## Falsification condition

The proposal should be weakened or rejected if, under matched budgets and representative tasks, it fails to outperform substantially simpler independent aggregation or produces reliability gains too small to justify its latency, complexity, and verification cost.
