---
id: GONI-EVIDENCE-SEMROUTER-01
title: Heterogeneous routing signals can be composed before model selection
type: evidence
status: draft
implementation_state: not_applicable
proposition: "The vLLM Semantic Router work demonstrates a routing architecture that composes heterogeneous deterministic and neural request signals before selecting among heterogeneous model backends, supporting Goni's separation of signal production from route selection."
domains:
- research
- models
- routing
aliases: []
relations:
- type: supports
  target: ROUTE-SIGNAL-01
- type: supports
  target: GONI-IMAP-EB2133E6965D
sources:
- SRC-LIU2026-VLLM-SEMANTIC-ROUTER
artifacts: []
uncertainty: "The source describes its own production architecture and reported capabilities. It does not establish that the same signal vocabulary, routing algorithms, or deployment model is optimal for Goni."
legacy: []
---

# Heterogeneous routing signals can be composed before model selection

The vLLM Semantic Router architecture extracts heterogeneous request signals
including heuristic features, context length, language, role-based
authorization, embeddings, domain classification, grounding, modality,
privacy, and safety signals, then composes them into routing decisions.

This supports the architectural separation in ROUTE-SIGNAL-01:

signal production -> signal composition -> route selection.

The transferable insight is the separation of typed signal producers from the
model-selection mechanism, not the adoption of a specific router or provider
stack.

Goni extends this pattern by keeping authority state kernel-owned and treating
probabilistic routing evidence as cognition rather than permission.
