---
id: GONI-EVIDENCE-SPECIALIST-GUARD-01
title: Specialized guard models demonstrate role-specific model allocation
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Llama Guard and ShieldGemma demonstrate that general language-model substrates can be specialized and evaluated for narrow guard or moderation roles, supporting MODEL-ROLE-01's separation of cognitive role from model identity."
domains:
- research
- models
- security
aliases: []
relations:
- type: supports
  target: MODEL-ROLE-01
- type: supports
  target: GONI-PRINCIPLE-HET-INTEL-01
sources:
- SRC-INAN2023-LLAMA-GUARD
- SRC-GOOGLE-SHIELDGEMMA-MODEL-CARD
artifacts: []
uncertainty: "Safety classification is only one specialist role. The cited evaluations do not establish universal guard accuracy, calibration, adversarial robustness, or suitability for Goni policy decisions."
legacy: []
---

# Specialized guard models demonstrate role-specific allocation

Llama Guard is an instruction-tuned language model specialized for classifying
prompts and responses against configurable safety-risk taxonomies.

ShieldGemma similarly provides model variants specialized for safety-content
moderation categories.

These systems demonstrate that a model need not act as a general conversational
agent merely because its substrate is generative. The same broad model family
can be exposed and evaluated through a narrower specialist role.

For Goni, this supports the MODEL-ROLE-01 guard role and the wider principle of
selecting cognition by function.

A guard prediction is still probabilistic cognition. It may inform routing,
verification, or policy inputs, but it does not itself become kernel policy or
authority.
