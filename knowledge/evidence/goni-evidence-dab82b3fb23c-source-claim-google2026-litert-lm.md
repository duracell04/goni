---
id: GONI-EVIDENCE-DAB82B3FB23C
title: 'Source claim: Google LiteRT-LM 2026'
type: evidence
status: draft
implementation_state: not_applicable
proposition: LiteRT-LM supports stateful on-device inference through session save and restore, KV-cache persistence, memory-local execution, and dynamic loading of model components.
domains:
- research
- architecture
aliases: []
relations:
- type: supports
  target: GONI-SPECIFICATION-BB656A72DF7D
sources:
- SRC-GOOGLE2026-LITERT-LM
artifacts: []
uncertainty: The source is a Google engineering publication and includes vendor-reported performance claims. It supports the existence of state-continuity mechanisms, not GONI's semantic-memory or authority rules.
legacy: []
---

# Source claim: Google LiteRT-LM 2026

## Reported observation

Google describes LiteRT-LM as supporting stateful sessions whose context can be saved and restored, including serialization of large KV-cache state. The same engineering material describes memory-local execution and dynamic loading of model components so active memory use can vary with the current task.

## GONI interpretation

These mechanisms show that local inference state need not be permanently resident to remain recoverable. They are therefore relevant implementation precedents for GONI's broader rehydration concept and for separating active inference state from recoverable system state.

## Limitation

A persisted KV cache is neither a provenance-bearing source archive nor a governed semantic memory system. LiteRT-LM does not establish GONI's retrieval, retention, permission, or authorization semantics, and vendor performance numbers require independent reproduction before use as benchmark evidence.
