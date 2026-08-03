---
id: GONI-IMAP-823922ED130C
title: Plane ℰ – Execution (append-only telemetry)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Concepts: tracing spans, LLM call billing, aggregated metrics.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/30-plane-contracts.md
  heading: Plane ℰ – Execution (append-only telemetry)
  revision: dcbe5931107b72f6a6af295e9e1b943accb6a2f9
---

# Plane ℰ – Execution (append-only telemetry)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Plane ℰ – Execution (append-only telemetry)
- Concepts: tracing spans, LLM call billing, aggregated metrics.
- Tables: LlmCalls, Metrics. (Spans optional later.)
- Allowed FK targets: may reference `request_id`, `context_id`, `task_id`, `span_id`.
- Forbidden: `LargeUtf8`; mutable updates (append-only semantics preferred).
