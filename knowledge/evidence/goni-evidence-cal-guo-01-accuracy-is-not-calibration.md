---
id: GONI-EVIDENCE-CAL-GUO-01
title: Accuracy does not establish probability calibration
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Guo et al. (2017) provide empirical evidence that modern neural networks can be accurate while their reported confidence is poorly calibrated, so predictive accuracy alone is insufficient evidence that confidence values represent empirical correctness likelihood."
domains:
- research
- models
- evaluation
aliases: []
relations:
- type: supports
  target: CAL-01
sources:
- SRC-GUO2017-CALIBRATION
artifacts: []
uncertainty: "The experiments study specific supervised classification architectures and datasets from 2017. They establish the conceptual distinction between accuracy and calibration but do not establish calibration behavior for current language or decision models."
legacy: []
---

# Accuracy does not establish probability calibration

Guo et al. evaluate calibration in modern neural networks and report that high
predictive accuracy can coexist with poor confidence calibration. They also
evaluate several post-hoc calibration approaches.

For Goni, the supported inference is deliberately narrow:

- correctness rate and probability calibration are separate empirical properties;
- a numeric confidence field should not be treated as a calibrated probability
  without evaluation;
- calibration procedures require dataset- and model-scoped validation.

This source does not establish that any single calibration method is universally
optimal, nor does it directly evaluate contemporary language models or bounded
semantic decision models.
