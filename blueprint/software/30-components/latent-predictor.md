# Latent predictor (JEPA-style) integration

This document describes a **latent-first** cognition pattern compatible with Goni's planes, queues, and memory guarantees. It is not a requirement to adopt a specific training regime; it is an interface and execution shape that allows multiple model backends.

## Summary

- **Encoders** map inputs to latent representations (vectors + minimal structured features).
- A **Predictor** updates latent state and selects actions (tools, memory, retrieval, planning).
- A **Decoder** is optional and used for language output and human-facing artifacts.

The system optimizes for correct latent state updates and correct tool actions, not for constant token emission.

## Block diagram (VL-JEPA inspired)

Inputs:
- Visual input `X_v` (image/screen/video)
- Text query `X_q` (user intent, task goal)
- Text target `Y` (optional during training; e.g., answer/draft)

Modules:
- `E_x`: X-Encoder maps `X_v -> S_v`
- `E_q`: Query encoder maps `X_q -> S_q` (or reuse text encoder)
- `P`: Predictor maps `(S_v, S_q, S_state) -> S_y_hat + S_state' + actions`
- `E_y`: Y-Encoder maps `Y -> S_y` (training / supervision only)
- `D_y`: Y-Decoder maps `S_y_hat -> text` (optional runtime)

Loss (training-time conceptual):
- compare `S_y_hat` vs `S_y` in latent space (similarity / contrastive loss)

Runtime:
- `E_y` and explicit loss are absent; only encoders + predictor + optional decoder run.

## Query conditioning

The query/goal `X_q` is treated as a conditioning signal:
- selects which aspects of `S_v` matter,
- constrains what retrieval is relevant,
- narrows what "correct" latent update looks like.

In Goni terms, `X_q` can be:
- a user message,
- a system goal,
- a policy constraint,
- or a tool result that changes the task.

## Relationship to RAG (retrieval is a tool, not cognition)

RAG is represented as tool calls and Memory Plane queries:
- encoders produce embeddings for the query and current state,
- predictor decides whether to retrieve,
- retrieved items are merged into latent state as evidence.

Key point: retrieval augments state; it does not replace the predictor. The predictor decides:
- what to retrieve (scope),
- how to weight evidence,
- when to stop retrieving.

## Relationship to planes

- **Data Plane (\(\mathcal{A}\) / \(\mathcal{X}\))**: observations, files, snapshots, events.
- **Memory Plane**: vector store, structured facts, latent summaries, timelines.
- **Control Plane (\(\mathcal{K}\))**: policies about privacy, tool permissions, escalation rules.
- **Execution Substrate (\(\mathcal{E}\))**: orchestrates encoder/predictor/decoder workers as queued jobs.

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

## Why this matters locally

- Always-on components can be compact (encoders + predictor).
- Decoder can be invoked only when output is needed.
- Latent summaries reduce raw-text duplication and improve privacy boundaries.


## Upstream
- [Latent state contract](/blueprint/software/docs/specs/latent-state-contract.md)
- [Symbolic substrate](/blueprint/software/docs/specs/symbolic-substrate.md)

## Downstream
- [Scheduler and interrupts](/blueprint/software/docs/specs/scheduler-and-interrupts.md)
- [ITCR](/blueprint/software/docs/specs/itcr.md)

## Adjacent
- [Vector DB](/blueprint/software/30-components/vecdb.md)
- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)
