---
id: GONI-EVIDENCE-SELECTIVE-01
title: Selective prediction exposes a risk-coverage trade-off
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Selective prediction literature formalizes systems that abstain on part of the input space and evaluates the resulting trade-off between coverage and predictive risk."
domains:
- research
- evaluation
- models
aliases: []
relations:
- type: supports
  target: SELECTIVE-01
sources:
- SRC-GEIFMAN2019-SELECTIVENET
artifacts: []
uncertainty: "SelectiveNet is one concrete learned approach. Goni adopts the general reject-option and risk-coverage concepts without requiring its architecture or training procedure."
legacy: []
---

# Selective prediction exposes a risk-coverage trade-off

Geifman and El-Yaniv study predictive systems with a reject option: the system
answers a subset of cases and abstains on the remainder. Performance is
therefore characterized jointly by coverage and the risk incurred on accepted
cases.

For Goni, the supported abstraction is the general selective-prediction
problem. A selector may be implemented by thresholds, learned models, rules, or
other mechanisms. The cited work does not establish which selector is optimal
for Goni workloads.
