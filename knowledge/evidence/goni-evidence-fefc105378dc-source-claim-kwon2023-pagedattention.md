---
id: GONI-EVIDENCE-FEFC105378DC
title: 'Source claim: Kwon et al. 2023 PagedAttention'
type: evidence
status: draft
implementation_state: not_applicable
proposition: PagedAttention demonstrates that LLM serving can decouple logical KV-cache organization from contiguous physical memory placement by applying paging and virtual-memory concepts.
domains:
- research
- architecture
aliases: []
relations:
- type: supports
  target: GONI-THESIS-AEA4F8746318
sources:
- SRC-KWON2023-VLLM
artifacts: []
uncertainty: The source concerns KV-cache memory management and serving throughput. Its relevance to cognitive memory is analogical and architectural, not direct empirical validation of GONI's semantic or governance model.
legacy: []
---

# Source claim: Kwon et al. 2023 PagedAttention

## Reported observation

Kwon et al. describe PagedAttention, which partitions KV-cache state into blocks and manages their placement using concepts analogous to operating-system virtual memory. The design reduces fragmentation and enables flexible sharing of KV-cache blocks across sequences.

## GONI interpretation

The result is a systems precedent for separating logical availability from physical residency. GONI applies that separation at a different layer: active model context is treated as one residency tier within a larger recoverable cognitive state.

## Limitation

PagedAttention does not establish that semantic memories, user evidence, permissions, or authority should be paged in the same manner. Those are independent GONI design claims requiring their own evaluation.
