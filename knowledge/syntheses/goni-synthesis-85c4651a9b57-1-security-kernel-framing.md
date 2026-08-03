---
id: GONI-SYNTHESIS-85C4651A9B57
title: 1. Security-kernel framing
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Goni is modeled as a reference-monitor style kernel for agent actions: every effectful action must be mediated, untrusted components must not bypass policy, the trusted core must stay small enough to test and audit.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/20-trust-model.md
  heading: 1. Security-kernel framing
  revision: 628398028a2ae5fe5696b6b3ec004da2314ddd96
---

# 1. Security-kernel framing

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Security-kernel framing

Goni is modeled as a reference-monitor style kernel for agent actions:
- every effectful action must be mediated,
- untrusted components must not bypass policy,
- the trusted core must stay small enough to test and audit.

This is design intent for the blueprint stage.
