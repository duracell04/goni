---
id: GONI-PRINCIPLE-EVAL-INDEP-01
title: "Evaluator Independence And Correlated Error"
type: principle
status: draft
implementation_state: specified_only
proposition: "Confidence should increase with independent information rather than raw evaluator count; models or agents that share evidence, training distributions, prompts, tools, or reasoning patterns may produce strongly correlated errors."
domains:
- evaluation
- models
aliases: []
relations:
- type: refines
  target: GONI-LAB
- type: depends_on
  target: GONI-PRINCIPLE-VER-EVID-01
sources: []
artifacts: []
uncertainty: "Effective evaluator independence is difficult to estimate directly; diversity dimensions are proxies that require empirical calibration."
legacy: []
---

# Evaluator Independence And Correlated Error

A council vote is informative only to the extent that its errors are independent.

[
N agreeing evaluators 
eq N independent observations
]

Goni should record and, where useful, diversify the dimensions that can generate independent information: model family, evidence source, retrieval corpus, tool access, verifier type, execution environment, search branch, and assigned objective.

Council confidence should be based on evidence diversity and observed calibration rather than simple majority size.
