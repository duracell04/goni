---
id: GONI-EXPERIMENT-9C433FEA9457
title: 7. Evaluation protocol suggestions
type: experiment
status: draft
implementation_state: not_applicable
proposition: Non-bypass tests for signing path prove no signature can be produced outside mediation boundary.
domains:
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/45-kernel-blockchain-mapping.md
  heading: 7. Evaluation protocol suggestions
  revision: 78a9ea426f651fe244b7cbb39f7603af04fe10b2
---

# 7. Evaluation protocol suggestions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Evaluation protocol suggestions

1. Non-bypass tests for signing path
- prove no signature can be produced outside mediation boundary.

2. Capability attenuation tests
- verify delegated signer rights cannot exceed parent authority.

3. Anchoring integrity tests
- verify commitment proofs for "receipt existed before time T".

4. Privacy leakage tests
- confirm anchored commitments reveal no sensitive payload.

5. Validator safety tests
- replay slashable scenarios and verify refusal invariants.
