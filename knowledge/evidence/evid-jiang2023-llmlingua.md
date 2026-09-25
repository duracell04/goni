---
id: EVID-JIANG2023-LLMLINGUA
title: 'Source claim: LLMLingua'
type: evidence
status: draft
implementation_state: not_applicable
proposition: LLMLingua introduces budget-controlled prompt compression and reports large prompt reductions with limited performance loss on its evaluated workloads.
domains:
- research
- software
relations:
- type: supports
  target: GONI-SPEC-A29366F3E2EA
sources:
- SRC-JIANG2023-LLMLINGUA
artifacts: []
uncertainty: Compression quality is workload- and model-dependent. Reported compression ratios are evidence for evaluation, not production thresholds for Goni.
legacy: []
---

# Source claim: LLMLingua

LLMLingua uses coarse-to-fine prompt compression with an explicit budget
controller intended to retain task-relevant information under aggressive token
reduction.

## Goni relevance

The paper supports treating token budget and compression policy as first-class
context-assembly variables rather than assuming that all retrieved text should
enter the prompt verbatim.

## Boundary

Goni must separately measure decisive-evidence recall, citation fidelity,
authority-layer safety, latency, and compression loss on representative tasks.
