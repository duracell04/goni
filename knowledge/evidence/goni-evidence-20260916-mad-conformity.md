---
id: GONI-EVIDENCE-20260916-MAD-CONFORMITY
title: Multi-agent debate can amplify persuasion, conformity, and correlated error
type: evidence
status: draft
implementation_state: not_applicable
proposition: Recent multi-agent LLM studies report that unconstrained interaction can propagate errors, induce conformity, and fail to outperform simpler independent aggregation reliably.
domains:
- epistemics
- multi-agent
- reasoning
aliases: []
relations: []
sources:
- SRC-KRAIDIA2026-ADVERSARIAL-MAD
- SRC-CUI2026-FREE-MAD
- SRC-SMIT2024-MAD
- SRC-CHOI2025-GROUP-CONFORMITY
- SRC-OKAWA2026-BIASED-CONSENSUS
artifacts: []
uncertainty: The cited studies use different models, tasks, debate protocols, adversarial assumptions, and evaluation regimes. They support a structural risk claim about interaction and conformity, not a universal claim that debate is inferior on every task.
legacy: []
---

# Multi-agent debate can amplify persuasion, conformity, and correlated error

## Reported observations

The contemporary multi-agent debate literature does not support the simple proposition that more interaction monotonically improves epistemic quality.

- Kraidia et al. (2026) report that a strategically persuasive adversarial agent can materially reduce collective accuracy and increase agreement on incorrect answers.
- Cui et al. (2026) identify conformity-driven error propagation as a failure mode of consensus-oriented debate and propose a consensus-free alternative.
- Smit et al. (2024) find that evaluated debate protocols do not reliably outperform simpler self-consistency and independent ensembling baselines without careful protocol tuning.
- Choi et al. (2025) report conformity effects in multi-agent interactions, including influence from numerical majorities and stronger agents.
- Okawa (2026) develops a physics-inspired model of biased collective consensus and reports that heterogeneity can suppress the transition. This item is a preprint and should be weighted accordingly.

## Goni interpretation

These findings support treating **inter-agent influence as an epistemic coupling channel**, not as evidence by itself. Interaction can transmit useful criticism, but it can also transmit rhetoric, shared misconceptions, status-like signals, and majority pressure. A high-reliability architecture should therefore control when agents observe one another and should preserve pre-interaction commitments for later comparison.

## Limitations

The evidence does not establish that all debate is harmful, that independent voting is always superior, or that a fixed number of deliberation rounds is optimal. Protocol effects are task- and model-dependent. The relevant architectural conclusion is narrower: **collective reasoning quality depends on the structure of information flow, not merely on the number of agents or discussion rounds**.
