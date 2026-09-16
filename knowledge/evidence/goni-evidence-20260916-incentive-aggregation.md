---
id: GONI-EVIDENCE-20260916-INCENTIVE-AGGREGATION
title: Epistemic aggregation benefits from outcome-linked scoring and explicit mechanism assumptions
type: evidence
status: draft
implementation_state: not_applicable
proposition: When outcomes become observable, proper scoring rules provide principled incentives for probabilistic honesty; when ground truth is unavailable, peer-prediction mechanisms are possible but rely on stronger assumptions.
domains:
- epistemics
- mechanism-design
- calibration
aliases: []
relations: []
sources:
- SRC-GNEITING2007-PROPER-SCORING
- SRC-PRELEC2004-BAYESIAN-TRUTH-SERUM
- SRC-LADHA1995-CORRELATED-VOTES
artifacts: []
uncertainty: LLMs do not automatically behave like strategic human forecasters with stable utilities or common priors. Mechanism-design results therefore motivate scoring architecture but do not transfer without empirical validation.
legacy: []
---

# Epistemic aggregation benefits from outcome-linked scoring and explicit mechanism assumptions

## Reported foundations

Gneiting and Raftery (2007) formalize proper and strictly proper scoring rules for probabilistic forecasts. Such rules are appropriate when the relevant outcome is eventually observed and the system can evaluate whether stated probabilities were calibrated.

Prelec (2004) develops the Bayesian Truth Serum for eliciting truthful subjective information when objective truth is not directly available. Its guarantees depend on assumptions about signal structure and beliefs and should not be transplanted mechanically into an LLM system.

Ladha (1995) provides a complementary warning: aggregation benefits decline when individual judgments are correlated.

## Goni interpretation

A multi-agent architecture induces incentives through its prompts, selection rules, reputation updates, and evaluation criteria even when the underlying models do not possess human-like enduring preferences. Rewarding agreement or selection can favor persuasive convergence. Rewarding later-validated correctness, calibrated probability, useful falsification, and accurately detected defects better aligns the evaluation layer with epistemic performance.

Where objective outcomes exist, Goni can accumulate domain-specific calibration histories using proper scoring rules. Where outcomes do not exist, peer-prediction ideas remain a research direction rather than a default mechanism.
