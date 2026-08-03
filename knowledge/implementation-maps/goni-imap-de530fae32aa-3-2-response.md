---
id: GONI-IMAP-DE530FAE32AA
title: 3.2 Response
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Non-streaming**: Single JSON object containing: id, created, model, choices[0].message (final assistant message), usage (prompt/completion token counts), optional goni_work_order_ref and goni_interaction_mode.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 3.2 Response
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 3.2 Response

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Response

**Non-streaming**:

* Single JSON object containing:

  * id, created, model,
  * choices[0].message (final assistant message),
  * usage (prompt/completion token counts),
  * optional `goni_work_order_ref` and `goni_interaction_mode`.

**Streaming**:

* Server-sent events (SSE):

  * Each event has a data: line with a JSON payload (choices[0].delta).
  * The stream ends with data: [DONE].
