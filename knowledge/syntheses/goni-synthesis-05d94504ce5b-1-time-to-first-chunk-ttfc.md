---
id: GONI-SYNTHESIS-05D94504CE5B
title: 1) Time to first chunk (TTFC)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Time to first streamed bytes reaching the client (HTTP headers, first SSE event, keep-alive chunk, or a "stream started" marker).
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: 1) Time to first chunk (TTFC)
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# 1) Time to first chunk (TTFC)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1) Time to first chunk (TTFC)

Time to first streamed bytes reaching the client (HTTP headers, first SSE
event, keep-alive chunk, or a "stream started" marker).

This mostly reflects:
- network + TLS + routing
- request admission
- "something started streaming"

Important: the first chunk is not always "the model is generating." Some stacks
emit an early "working..." chunk before model compute begins.
