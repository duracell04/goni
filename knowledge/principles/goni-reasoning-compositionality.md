---
id: GONI-PRINCIPLE-REASON-COMPOSE
title: Compose reasoning frameworks across domain boundaries
type: principle
status: draft
implementation_state: not_applicable
proposition: Select and combine theories by the structure of a decision problem and its evidence, allowing multiple lenses to contribute without assigning exclusive ownership to application domains.
domains:
  - reasoning
  - system
aliases: []
relations:
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-GAME
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-AGENCY
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-ATTENTION
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-CAUSAL
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-CONTROL
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-OPT
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-NETWORK
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-MECHANISM
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-DYNAMICS
sources: []
artifacts: []
uncertainty: No universal optimal theory-composition algorithm has been established.
legacy: []
---

# Compositional reasoning

Problems can simultaneously involve strategic incentives, asymmetric information, resource scarcity, causal uncertainty, and feedback. Use the decision structure and available evidence to select relevant theories, then combine their distinct contributions. For example, optimization identifies an efficient allocation under assumed rules; mechanism design asks whether actors will follow the allocation; control theory examines feedback stability; causal inference tests whether an intervention produced the observed result. Theories remain independently addressable nodes with typed relationships, assumptions and limitations.

**Selection contract:** input problem and evidence → candidate lenses → assumptions/overlaps → jointly consistent analysis → decision options → tests and uncertainty. Preserve contradictions as explicit reviewable hypotheses.

**Avoid category silos:** law, investing, AI engineering, negotiation, psychology and research are application contexts, not exclusive partitions of the underlying reasoning knowledge.
