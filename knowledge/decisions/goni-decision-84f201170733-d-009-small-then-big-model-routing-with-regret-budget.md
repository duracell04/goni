---
id: GONI-DECISION-84F201170733
title: D-009 – Small-then-big model routing with regret budget
type: decision
status: draft
implementation_state: specified_only
proposition: "Model routing should be evaluated as a contextual decision problem in which regret, calibration, selective risk, consequence-sensitive loss, latency, cost, privacy, energy, and authority constraints are measured explicitly; any numeric regret target is benchmark-specific rather than a universal architectural constant."
domains:
- software
aliases: []
relations:
- type: depends_on
  target: CAL-01
- type: depends_on
  target: SELECTIVE-01
- type: depends_on
  target: LOSS-01
- type: refines
  target: GONI-IMAP-EB2133E6965D
sources:
- SRC-FRUGALGPT2023
- SRC-ROUTELLM2024
- SRC-ROUTERBENCH2024
artifacts: []
uncertainty: "The original migrated draft used 0.07 as a default average-regret target without repository evidence deriving that value. This revision preserves regret as a metric while demoting numeric targets to benchmark-specific configuration."
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-009 – Small-then-big model routing with regret budget
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-009 – Small-then-big model routing with regret budget

The minimal pedagogical routing problem distinguishes:

- a_s: accept the result from a cheaper/smaller route;
- a_l: escalate to a stronger/more expensive route.

Real deployments MAY use a larger action set including deterministic checks,
bounded decision models, local models, remote models, multi-model verification,
or principal review.

For a policy pi and an oracle comparator pi*, cumulative regret may be written:

R_T = sum from t=1..T of [L(pi(x_t), y_t) - L(pi*(x_t), y_t)],

with the exact loss definition and comparator recorded by the benchmark.

Regret remains useful because it quantifies the cost of approximate routing
against a declared oracle. It is not sufficient on its own.

Router evaluation SHOULD additionally report, where applicable:

- predictive calibration;
- risk-coverage behavior;
- consequence-sensitive expected loss;
- false local accepts and late escalations;
- latency and monetary cost;
- local energy/thermal cost where measurable;
- privacy and egress exposure;
- trajectory-level economics.

The historical draft target of average regret <= 0.07 is retained only as a
possible benchmark configuration when explicitly declared. It is not a
universal invariant and MUST NOT be interpreted as theoretically justified
across tasks.

A routing threshold or regret target is promotable only relative to a specified
dataset, task distribution, oracle construction, loss function, consequence
class, and model/harness version.

Hard policy and authority constraints remain outside the regret trade-off:
economic or predictive advantage cannot authorize a prohibited action.
