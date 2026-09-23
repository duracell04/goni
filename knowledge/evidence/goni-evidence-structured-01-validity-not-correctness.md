---
id: GONI-EVIDENCE-STRUCTURED-01
title: Output constraints establish form, not semantic correctness
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Grammar-constrained decoding can restrict generation to structurally legal outputs while task accuracy remains a separate empirical property, supporting a strict separation between schema validity and semantic correctness."
domains:
- research
- models
- assurance
aliases: []
relations:
- type: supports
  target: GONI-PRINCIPLE-VALIDITY-CORRECTNESS-01
sources:
- SRC-GENG2023-GRAMMAR-CONSTRAINED-DECODING
artifacts: []
uncertainty: "The cited paper studies grammar-constrained generation on structured NLP tasks; the general assurance distinction also applies beyond that specific decoding method."
legacy: []
---

# Output constraints establish form, not semantic correctness

Grammar constraints can ensure that generated strings belong to a permitted
formal language. The resulting output may nevertheless encode the wrong class,
argument, value, or decision for the underlying task.

For Goni, this supports separate assurance claims for structure and meaning.
A zero malformed-output rate is not evidence of zero semantic error.
