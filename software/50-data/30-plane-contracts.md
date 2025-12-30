# 30 – Plane Contracts (𝒜, 𝒳, 𝒦, ℰ)

Defines responsibilities, allowed foreign keys, and forbidden field types per plane. Arrow types live in `51-schemas-mvp.md`; this document is conceptual.

## Plane 𝒜 – Knowledge (immutable corpus)
- Concepts: `Doc`, `Chunk`, `Embedding`.
- Tables: Docs, Chunks, Embeddings, StateSnapshots, StateDeltas, LatentSummaries.
- Allowed FK targets from other planes: `Chunks.chunk_id` (referenced by 𝒳.ContextItems), `Embeddings.chunk_id`.
- Forbidden: no inbound FK from ℰ to `text`; no mutable state.

## Plane 𝒳 – Context (ephemeral)
- Concepts: live prompt text and selected retrieval units.
- Tables: ContextItems (MVP). Prompts/Contexts may be materialised later.
- Allowed FK targets: `context_id` may be referenced by ℰ.LlmCalls; `chunk_id` references 𝒜.Chunks.
- Forbidden: persistence beyond retention window; no sharing of raw text outside 𝒳.

## Plane 𝒦 – Control (metadata only)
- Concepts: requests, scheduling state.
- Tables: Requests, Tasks, AuditRecords, CapabilityTokens, AgentManifests. (QueueSnapshot, RouterDecisions optional later.)
- Allowed FK targets: `request_id` referenced by 𝒳 (Prompts/Contexts) and ℰ (LlmCalls); `task_id` referenced by ℰ spans.
- Forbidden: `LargeUtf8` fields; raw text never stored here.

## Plane ℰ – Execution (append-only telemetry)
- Concepts: tracing spans, LLM call billing, aggregated metrics.
- Tables: LlmCalls, Metrics. (Spans optional later.)
- Allowed FK targets: may reference `request_id`, `context_id`, `task_id`, `span_id`.
- Forbidden: `LargeUtf8`; mutable updates (append-only semantics preferred).
