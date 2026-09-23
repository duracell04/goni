---
id: GONI-PRINCIPLE-VALIDITY-CORRECTNESS-01
title: Structural Validity Is Not Semantic Correctness
type: principle
status: draft
implementation_state: specified_only
proposition: "Deterministic schema or grammar validity establishes that an output belongs to the permitted representation space; semantic correctness remains a separate empirical or evidentiary property."
domains:
- assurance
- models
- system
aliases: []
relations:
- type: refines
  target: DECISION-MODEL-01
- type: supports
  target: GONI-PRINCIPLE-HET-INTEL-01
sources:
- SRC-GENG2023-GRAMMAR-CONSTRAINED-DECODING
artifacts: []
uncertainty: "Semantic correctness may itself be deterministic for some domains when a complete formal verifier exists. The distinction concerns what a structural validator alone proves."
legacy: []
---

# Structural Validity Is Not Semantic Correctness

Let Y be the permitted output space and let y* denote the correct task answer.

Structural validity establishes y in Y.

Semantic correctness establishes y = y*.

In general:

y in Y does not imply y = y*.

Therefore the repository distinguishes:

- parser/schema/grammar validity;
- semantic decision quality;
- probability calibration;
- policy authorization.

Each property requires its own evidence or deterministic proof path.

A bounded decision model with no malformed outputs may still select the wrong
legal decision. Likewise, a generative model constrained to valid JSON can
still produce semantically false JSON.
