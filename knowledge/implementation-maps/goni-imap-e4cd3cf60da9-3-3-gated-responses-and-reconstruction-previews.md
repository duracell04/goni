---
id: GONI-IMAP-E4CD3CF60DA9
title: 3.3 Gated responses and reconstruction previews
type: implementation-map
status: draft
implementation_state: specified_only
proposition: If a request crosses a soft/hard gate, the API MAY return a reconstruction preview instead of executing immediately.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 3.3 Gated responses and reconstruction previews
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 3.3 Gated responses and reconstruction previews

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Gated responses and reconstruction previews

If a request crosses a soft/hard gate, the API MAY return a reconstruction
preview instead of executing immediately. The preview must be kernel-backed and
compact:

- `Goal`
- `Done`
- `Assumptions`
- `Risk`
- `Question`

Logical response additions:

- `goni_work_order_ref`
- `goni_reconstruction`
- `goni_requires_approval`

> **Invariant API-2 (streaming monotonicity)**
> For a given request \(R\), the concatenation of all streamed delta.content fragments per choice must equal the message.content of the corresponding non-streaming response (up to tokenisation whitespace).

---
