---
id: GONI-EVIDENCE-20260916-EVALUATOR-BIAS
title: LLM evaluators exhibit self-preference and presentation-order bias
type: evidence
status: draft
implementation_state: not_applicable
proposition: LLM-based evaluation is itself a fallible inference process and can be biased by self-recognition and candidate presentation order.
domains:
- epistemics
- evaluation
- multi-agent
aliases: []
relations: []
sources:
- SRC-PANICKSSERY2024-SELF-PREFERENCE
- SRC-SHI2025-POSITION-BIAS
artifacts: []
uncertainty: Bias magnitude depends on model, task, prompt, comparison format, and candidate distribution. Blinding and randomization mitigate specific channels but do not make evaluation unbiased.
legacy: []
---

# LLM evaluators exhibit self-preference and presentation-order bias

## Reported observations

Panickssery, Bowman, and Feng (2024) report that LLM evaluators can recognize their own generations above chance and that self-recognition is associated with self-preference. Shi et al. (2025) report systematic position bias across pairwise and list-wise LLM-as-a-judge settings.

## Goni interpretation

Removing explicit author labels is useful but insufficient. A reviewer may still infer model family or authorship from stylistic features, and an evaluator may be influenced by candidate order.

High-reliability review should therefore separate **provenance retention** from **provenance visibility**:

- preserve complete authorship and generation provenance in the system record;
- hide nonessential identity signals from the reviewer during first-pass evaluation;
- randomize candidate ordering where comparison order is not semantically meaningful;
- normalize the response schema enough to reduce superficial presentation effects without rewriting away substantive evidence;
- reveal provenance later when it is itself relevant evidence, for example for calibration histories or model-specific failure modes.

Blinding is therefore an information-control technique, not a guarantee of independence.
