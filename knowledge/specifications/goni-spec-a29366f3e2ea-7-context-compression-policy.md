---
id: GONI-SPEC-A29366F3E2EA
title: 7. Context Compression Policy
type: specification
status: draft
implementation_state: specified_only
proposition: Context assembly often finds more relevant material than fits in the prompt window.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 7. Context Compression Policy
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 7. Context Compression Policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Context Compression Policy

Context assembly often finds more relevant material than fits in the prompt
window. A ContextPack MUST record the compression policy used for each selected
item when compression affects the prompt bundle.

Allowed compression forms:

| Form | Use |
| --- | --- |
| `raw_excerpt` | Source-grounded tasks that need exact wording or citations. |
| `summary` | General tasks where bounded prose is sufficient. |
| `latent_summary` | Compact state or derived memory where raw text should not be sent. |
| `decision_only` | Tasks that need the resulting decision or rule, not the full discussion. |
| `citation_only` | Tasks that need a waypoint/reference but not content in the model context. |

The Work Order type, risk class, output shape, permission scope, quoteability,
and token budget SHOULD drive compression choice. A legal memo may prefer
`raw_excerpt`; a style-sensitive social draft may prefer `summary` or
`latent_summary`; a high-risk action may include `citation_only` refs for
audit while withholding sensitive content from the model.
