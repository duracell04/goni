---
id: GONI-EVIDENCE-CAL-SCORING-01
title: Proper scoring rules evaluate probabilistic forecasts
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Gneiting and Raftery (2007) establish strictly proper scoring rules as principled evaluation functions for probabilistic forecasts, supporting the use of proper scores such as logarithmic and quadratic/Brier-style scores when assessing confidence-bearing decision models."
domains:
- research
- evaluation
aliases: []
relations:
- type: supports
  target: CAL-01
sources:
- SRC-GNEITING-RAFTERY2007-SCORING
artifacts: []
uncertainty: "A proper score evaluates probabilistic forecasts under its mathematical assumptions; it does not by itself guarantee calibration under distribution shift or establish an operational decision threshold."
legacy: []
---

# Proper scoring rules evaluate probabilistic forecasts

Gneiting and Raftery develop the theory of proper and strictly proper scoring
rules. Such rules evaluate predictive distributions rather than only top-label
correctness.

For Goni this supports evaluating confidence-bearing cognition with proper
scores in addition to classification accuracy. The concrete score must match
the prediction object and task.

Proper scoring is an evaluation foundation, not an authority mechanism and not
a guarantee that a forecast remains calibrated outside the evaluated
distribution.
