---
id: EVID-XGRAMMAR-STRUCTURED-GENERATION-01
title: 'Source claim: XGrammar enforces structured generation through constrained decoding'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Dong et al. present XGrammar as a grammar-constrained structured-generation engine that enforces output structure during decoding and is designed for efficient integration with LLM inference runtimes.
domains:
- research
- inference
- tools
aliases: []
relations:
- type: supports
  target: STRUCT-OUT-01
sources:
- SRC-DONG2024-XGRAMMAR
artifacts: []
uncertainty: Reported speedups and overhead depend on evaluated hardware, grammars, runtimes, and workloads; the source does not establish semantic correctness or action safety.
legacy: []
---

# Source claim: XGrammar enforces structured generation through constrained decoding

XGrammar applies context-free-grammar constraints during decoding so generated token sequences remain inside the permitted language.

For Goni, this supports moving structural validity into the inference/runtime layer when practical.

The source does not show that a grammar-valid tool call is semantically appropriate, based on fresh state, or authorized. Those remain separate Goni checks.
