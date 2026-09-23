---
id: GONI-EVIDENCE-DRIFT-01
title: Uncertainty quality can degrade under dataset shift
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Ovadia et al. (2019) provide empirical evidence that predictive uncertainty behavior can degrade under dataset shift, supporting distribution-scoped calibration evidence and explicit revalidation after material operating changes."
domains:
- research
- evaluation
- models
aliases: []
relations:
- type: supports
  target: CAL-DRIFT-01
sources:
- SRC-OVADIA2019-UNCERTAINTY-SHIFT
artifacts: []
uncertainty: "The source evaluates particular supervised learning methods and benchmark shifts. It supports the general need to test uncertainty under shift, not a universal drift detector for Goni."
legacy: []
---

# Uncertainty quality can degrade under dataset shift

Ovadia et al. evaluate predictive uncertainty as test distributions move away
from training conditions and report degradation under dataset shift.

For Goni this supports treating calibration as scoped evidence rather than a
permanent intrinsic property of a model. Material changes in the task,
population, model, harness, or input distribution can invalidate prior
operating-point evidence and should trigger re-evaluation.
