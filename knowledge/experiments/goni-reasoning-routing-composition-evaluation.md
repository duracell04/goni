---
id: GONI-EXPERIMENT-REASON-ROUTING
title: Evaluate atomic theory retrieval and compositional reasoning
type: experiment
status: draft
implementation_state: specified_only
proposition: Compare theory-aware graph retrieval and conditional solver/model routing against fixed-prompt and unstructured reasoning baselines on representative multi-theory decisions.
domains:
  - reasoning
  - system
aliases: []
relations:
  - type: tests
    target: GONI-PROPOSAL-REASON-ROUTER
sources: []
artifacts: []
uncertainty: Benchmark dataset, evaluation rubric, baseline configuration and cost limits are to be designed.
legacy: []
---

# Proposed experiment

Construct blinded tasks requiring one or several of game theory, principal–agent theory, rational inattention, causal inference, control theory, optimization, network theory, mechanism design, and dynamical/learning models. Annotate assumptions, correct outputs, and legitimate alternative decompositions independently.

Compare (A) unstructured model reasoning, (B) fixed all-framework checklist, (C) atomic retrieval/composition, and (D) retrieval plus deterministic solver routing. Match model, tools, context and compute budget. Measure task success, relevant framework recall/precision, assumption accuracy, verified numeric outputs, latency, token/energy cost, unnecessary complexity, and authority violations. Use held-out tasks and ablations; publish failures and confidence intervals. Stop or simplify when marginal quality fails to exceed marginal complexity and inference cost.
