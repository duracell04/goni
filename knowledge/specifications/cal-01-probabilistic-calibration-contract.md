---
id: CAL-01
title: Probabilistic Calibration Contract
type: specification
status: draft
implementation_state: specified_only
proposition: "A confidence value may be used as calibrated probability only when a versioned calibration record identifies the estimator, evaluation distribution, metric definitions, empirical results, and validity scope supporting that interpretation."
domains:
- evaluation
- models
- specs
aliases:
- CALIBRATION-CONTRACT
relations:
- type: refines
  target: GONI-IMAP-EB2133E6965D
- type: depends_on
  target: DECISION-MODEL-01
sources:
- SRC-GUO2017-CALIBRATION
- SRC-GNEITING-RAFTERY2007-SCORING
artifacts: []
uncertainty: "Calibration is distribution-, task-, estimator-, and version-dependent. No single metric or post-hoc calibration method is sufficient for every decision model or deployment."
legacy: []
---

# Probabilistic Calibration Contract

## 1. Accuracy and calibration are distinct

Accuracy concerns whether selected decisions are correct. Calibration concerns
whether reported probabilities correspond to empirical frequencies over a
defined evaluation population.

Accordingly:

\[
\text{accuracy} \neq \text{calibration}.
\]

A score in \([0,1]\) is not automatically a probability, and a probability is
not automatically calibrated.

## 2. Calibration record

Before calibrated confidence is used for routing, abstention, or
consequence-sensitive control, the system SHOULD have a versioned calibration
record containing at least:

- estimator and model/checkpoint identity;
- relevant harness, prompt, adapter, or decision-schema version;
- task and target-event definition;
- calibration and evaluation dataset identities;
- sampling period or snapshot date where relevant;
- in-domain population description;
- calibration method, if any;
- raw-score semantics;
- calibrated-probability semantics;
- metric definitions and parameters;
- empirical results;
- known limitations and validity scope.

## 3. Evaluation views

For probability-bearing classification or bounded-decision tasks, evaluation
SHOULD include multiple complementary views where applicable:

- reliability diagram or calibration curve;
- a proper scoring rule such as Brier/quadratic score for suitable discrete outcomes;
- negative log-likelihood/log score where supported;
- expected calibration error or another binned diagnostic, with binning recorded;
- ordinary task performance such as accuracy, F1, or task-specific loss.

No single metric is treated as a universal certificate.

## 4. Example diagnostics

A generic binned expected calibration error is

\[
\operatorname{ECE}
=
\sum_{b=1}^{B}
\frac{|I_b|}{n}
\left|
\operatorname{acc}(I_b)-\operatorname{conf}(I_b)
\right|.
\]

For binary forecasts with probability \(p_i\) and realized outcome
\(y_i \in \{0,1\}\), a Brier-style score is

\[
\operatorname{BS}
=
\frac{1}{n}\sum_{i=1}^{n}(p_i-y_i)^2.
\]

For a predicted distribution assigning probability \(P(y_i)\) to the
observed outcome,

\[
\operatorname{NLL}
=
-\frac{1}{n}\sum_{i=1}^{n}\log P(y_i).
\]

Metric direction, binning, normalization, and target event MUST be recorded in
the evaluation artifact.

## 5. Calibration methods

Post-hoc transformations MAY be evaluated, but no method is an architectural
default merely because it performs well in another model family or dataset.

The promoted calibration artifact is the evaluated mapping and its evidence,
not the name of a technique.

## 6. Authority boundary

Calibration can inform local acceptance, abstention, model escalation,
verification intensity, and review priority.

Calibration does not create capabilities or permission:

\[
\text{calibrated confidence} \not\Rightarrow \text{authority}.
\]

Canonical authority continues to resolve through kernel policy.
