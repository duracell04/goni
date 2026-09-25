---
id: EVID-LI2021-PREFIX-TUNING
title: 'Source claim: Prefix-Tuning'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Prefix-Tuning keeps base-model parameters frozen and learns continuous task-specific prefixes that subsequent tokens attend to as virtual tokens.
domains:
- research
- software
relations:
- type: supports
  target: GONI-SPEC-9D8A0A7F0F70
sources:
- SRC-LI2021-PREFIX-TUNING
artifacts: []
uncertainty: Learned prefixes remain model-specific learned state and do not replace explicit policy, provenance, or deterministic authority enforcement.
legacy: []
---

# Source claim: Prefix-Tuning

Prefix-Tuning optimizes continuous task-specific vectors while leaving the base
language model frozen.

## Goni relevance

The method provides evidence for a representation continuum between explicit
natural-language instructions and deeper learned adaptation.

## Boundary

A learned prefix can condition cognition; it is not an authority primitive and
must remain governed by model-bundle provenance and evaluation.
