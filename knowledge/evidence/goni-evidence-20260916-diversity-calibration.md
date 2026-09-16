---
id: GONI-EVIDENCE-20260916-DIVERSITY-CALIBRATION
title: Diversity and calibrated confidence are distinct from nominal agent count
type: evidence
status: draft
implementation_state: not_applicable
proposition: Collective reliability depends on the diversity and calibration of information sources; correlated agents and uncalibrated confidence can make nominal multiplicity overstate epistemic evidence.
domains:
- epistemics
- multi-agent
- calibration
aliases: []
relations: []
sources:
- SRC-ZHU2026-MAD-DIVERSITY
- SRC-BAND2024-LINGUISTIC-CALIBRATION
- SRC-LADHA1995-CORRELATED-VOTES
artifacts: []
uncertainty: Classical correlated-vote results concern human or abstract juries rather than LLMs directly. The effective-sample-size analogy is therefore a heuristic unless model-error dependence is measured empirically.
legacy: []
---

# Diversity and calibrated confidence are distinct from nominal agent count

## Reported observations

Zhu et al. (2026) identify diversity of initial viewpoints and explicit calibrated confidence communication as important mechanisms in multi-agent debate. Band et al. (2024) show that long-form language-model confidence is not automatically calibrated and develop a decision-oriented notion of linguistic calibration. In classical collective-decision theory, Ladha (1995) shows that positive correlation among votes weakens the information-pooling advantage associated with majority aggregation.

## Goni interpretation

The scarce resource in a multi-agent reasoning system is better described as **independent epistemic signal** than as raw agent count.

For an exchangeable set of $n$ signals with common pairwise correlation $\rho$, the familiar heuristic

$$
n_{\mathrm{eff}} \approx \frac{n}{1+(n-1)\rho}
$$

illustrates the point: when errors are highly correlated, additional nominal agents can add little independent information. This expression should not be interpreted as a general theorem for arbitrary LLM outputs; it is a diagnostic analogy that motivates empirical measurement of error dependence.

Likewise, a model's stated confidence should not receive direct epistemic weight merely because it is numerically high. Confidence becomes useful only to the extent that it is calibrated for the relevant task, model, domain, and operating regime.

## Implication

Goni should prefer **lineage diversity** over superficial multiplicity: different model families, retrieval paths, tools, data sources, prompts, and verification mechanisms can be more valuable than many replicas sharing the same failure modes.
