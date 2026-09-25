---
id: GONI-EVIDENCE-OPEN-DECISION-MODELS-20260926
title: Open implementations span dedicated decision models and lightweight bounded classifiers
type: evidence
status: draft
implementation_state: not_applicable
proposition: "By 26 September 2026, open repositories including Laya, Kev, Tev1, and GLiClass provide multiple implementation paths for bounded semantic decisions, supporting Goni's provider-independent DECISION-MODEL-01 abstraction rather than a vendor-specific Jev dependency."
domains:
- market
- models
- research
aliases: []
relations:
- type: supports
  target: DECISION-MODEL-01
- type: supports
  target: GONI-EXPERIMENT-DECISION-LAYER-01
sources:
- SRC-LAYA-SYSTEM-ONE-REPO
- SRC-KEV-DECISION-MODEL-REPO
- SRC-TOGETHER-TEV1-REPO
- SRC-GLICLASS-REPO
artifacts: []
uncertainty: "These projects differ materially in architecture, training, output semantics, maturity, and evaluation quality. Repository-reported benchmarks are not treated as independently reproduced results."
legacy: []
---

# Open bounded-decision implementations — 2026-09-26

The open landscape now spans several distinct implementation strategies.

Laya describes a non-autoregressive typed decision engine with choice, score,
and yes/no outputs and multiple questions evaluated against shared state.

Kev publishes Qwen-based research decision models with a dedicated pointer
readout, typed questions, probability distributions, calibration evaluation,
and explicit warnings about out-of-domain use.

Tev1 demonstrates another path: specialize a conventional small generative
model to choose among declared answer options while publishing the data and
training recipe.

GLiClass occupies the lighter discriminative end of the spectrum by performing
zero-shot classification over supplied labels in a single forward pass.

These projects strengthen the architectural claim that bounded semantic
inference should be an interface category rather than a Jev-specific feature.
They do not establish that one architecture dominates across Goni workloads.
