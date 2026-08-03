---
id: GONI-IMAP-17337DC4F852
title: Minimal interfaces (contracts)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Encoders should emit: embedding: float[n] features: {key: value} (small) provenance: {source, time, permissions} Predictor should accept: state_embedding (optional) observation_embeddings[] query_embedding policy_constraints Predictor should emit: state_embedding'' state_summary_struct (optional) actions[] (tool calls / retrieval requests / planning steps)'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/latent-predictor.md
  heading: Minimal interfaces (contracts)
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Minimal interfaces (contracts)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Minimal interfaces (contracts)

Encoders should emit:
- `embedding: float[n]`
- `features: {key: value}` (small)
- `provenance: {source, time, permissions}`

Predictor should accept:
- `state_embedding` (optional)
- `observation_embeddings[]`
- `query_embedding`
- `policy_constraints`

Predictor should emit:
- `state_embedding'`
- `state_summary_struct` (optional)
- `actions[]` (tool calls / retrieval requests / planning steps)
- `answer_embedding` (optional for decoder)

Decoder should accept:
- `answer_embedding` or `state_summary_struct`
- `style/format constraints`
- and emit final text or artifact.
