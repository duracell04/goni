---
id: EVID-ZHENG2024-SGLANG
title: 'Source claim: SGLang and RadixAttention'
type: evidence
status: draft
implementation_state: not_applicable
proposition: SGLang introduces RadixAttention to reuse KV cache across structured language-model program calls and reports throughput improvements on evaluated workloads.
domains:
- research
- software
relations:
- type: supports
  target: GONI-SYNTHESIS-227700D474C6
sources:
- SRC-ZHENG2024-SGLANG
artifacts: []
uncertainty: Reported throughput gains depend on workload, model, hardware, and runtime configuration and do not establish the cache-key or invalidation contract Goni should adopt.
legacy: []
---

# Source claim: SGLang and RadixAttention

SGLang treats KV reuse across structured generation calls as a runtime systems
problem and introduces RadixAttention for automatic prefix-aware cache reuse.

## Goni relevance

The result supports making prefix and KV reuse observable, explicit, and
runtime-aware rather than treating reuse as an opaque backend optimization.

## Boundary

Goni still needs its own compatibility identity, invalidation rules, provenance,
and authority separation.
