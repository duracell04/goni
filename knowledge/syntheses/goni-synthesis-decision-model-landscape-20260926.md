---
id: GONI-SYNTHESIS-DECISION-MODEL-LANDSCAPE-20260926
title: Bounded Decision-Model Landscape Snapshot — 2026-09-26
type: synthesis
status: draft
implementation_state: not_applicable
proposition: "The bounded semantic decision landscape now includes hosted and open implementations across dedicated non-generative decision models, constrained small language models, and lightweight classifiers, reinforcing a provider-independent interface and matched evaluation requirement."
domains:
- market
- models
- research
aliases: []
relations:
- type: supersedes
  target: GONI-SYNTHESIS-DECISION-MODEL-LANDSCAPE-20260923
- type: synthesizes
  target: GONI-EVIDENCE-JEV-20260923
- type: synthesizes
  target: GONI-EVIDENCE-OPEN-DECISION-MODELS-20260926
- type: synthesizes
  target: DECISION-MODEL-01
- type: synthesizes
  target: GONI-EXPERIMENT-DECISION-LAYER-01
sources:
- SRC-TYPESAFE2026-JEV
- SRC-LAYA-SYSTEM-ONE-REPO
- SRC-KEV-DECISION-MODEL-REPO
- SRC-TOGETHER-TEV1-REPO
- SRC-GLICLASS-REPO
artifacts: []
uncertainty: "The category remains young and terminology unstable. Architectures and benchmark protocols differ enough that provider claims cannot be compared as if they were one standardized leaderboard."
legacy: []
---

# Bounded Decision-Model Landscape Snapshot — 2026-09-26

The category is broader than one hosted product.

Current candidates illustrate at least three technical strategies:

1. dedicated bounded decision architectures that directly score declared
   options or question branches;
2. conventional small language models specialized to select among constrained
   options;
3. lightweight discriminative classifiers that accept runtime label sets.

The common architectural contract is:

state
-> bounded question or label space
-> typed semantic decision or predictive score
-> calibration/selective control where justified
-> deterministic software and kernel policy.

A hosted provider, open checkpoint, task-specific classifier, or constrained
small LLM may occupy this slot if matched evaluation supports it.

The architecture therefore remains provider-neutral. All candidates should be
benchmarked under GONI-EXPERIMENT-DECISION-LAYER-01 rather than promoted from
branding, architecture claims, or self-reported benchmark results alone.
