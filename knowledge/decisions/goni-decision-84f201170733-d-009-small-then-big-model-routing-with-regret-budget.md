---
id: GONI-DECISION-84F201170733
title: D-009 – Small-then-big model routing with regret budget
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** We treat the routing problem as a decision between: \(a_s\): answer with small model only.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-009 – Small-then-big model routing with regret budget
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-009 – Small-then-big model routing with regret budget

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-009 – Small-then-big model routing with regret budget

**Formal statement**

We treat the routing problem as a decision between:

- \(a_s\): answer with small model only.  
- \(a_\ell\): escalate to large model.

We define a **regret budget** \(\bar{R}\), and require that:
$$
\limsup_{T \to \infty} \frac{R_T}{T} \le \bar{R}
$$
with \(\bar{R} = 0.07\) by default.

Here \(R_T\) is regret vs an oracle policy that knows the “true” best action per request.

**Rationale**

- Makes the “small-first” heuristic quantifiable: we know how much quality we trade for speed/cost.  
- Provides a clear metric for validating router training and calibration.

**Consequence**

- Router changes must be evaluated on standard corpora with regret estimates.  
- “Always large model” is allowed as a configuration but is explicitly outside the regret accounting (it corresponds to the oracle upper bound on quality, not the baseline).

---
