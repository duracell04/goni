---
id: GONI-EVIDENCE-CONTEXT-COMPRESSION-01
title: Prompt-level semantic compression is distinct from source selection and KV retention
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Selective Context and LongLLMLingua provide empirical examples of reducing or reordering textual prompt material after source acquisition, supporting CONTEXT-ALLOC-01's explicit L2 semantic text-compression layer."
domains:
- research
- memory
- models
aliases: []
relations:
- type: supports
  target: CONTEXT-ALLOC-01
sources:
- SRC-LI2023-SELECTIVE-CONTEXT
- SRC-JIANG2024-LONGLLMLINGUA
artifacts: []
uncertainty: "Reported compression gains and quality trade-offs are task-, model-, compression-ratio-, and benchmark-specific. Compression can omit decisive evidence and must retain provenance/recovery paths where required."
legacy: []
---

# Prompt-level semantic compression is a distinct layer

Selective Context prunes redundant input context before LLM inference.
LongLLMLingua uses question-aware prompt compression and reorganization for
long-context tasks.

Both operate after material has already been acquired or selected for possible
use and before model-internal KV retention becomes the relevant resource
problem.

This supports CONTEXT-ALLOC-01's middle layer:

selected source material
-> semantic text retention/compression
-> model prefill.

The transfer to Goni is architectural rather than a commitment to either
algorithm. L2 compression should be evaluated separately from L1 retrieval and
L3 KV-cache policies so information-loss causes remain identifiable.
