---
id: GONI-SPEC-A29366F3E2EA
title: 7. Context Compression Policy
type: specification
status: draft
implementation_state: specified_only
proposition: Context assembly may compress selected material only while preserving the fidelity required by the Work Order; compression is a budgeted representation choice rather than an objective in itself.
domains:
- specs
aliases: []
relations: []
sources:
- SRC-JIANG2023-LLMLINGUA
- SRC-JIANG2024-LONGLLMLINGUA
- SRC-LI2023-SELECTIVE-CONTEXT
artifacts: []
uncertainty: Published prompt-compression gains are model- and workload-specific; Goni requires matched evaluation before promoting compression thresholds or strategies.
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
| `raw_excerpt` | Source-grounded tasks that need exact wording, citation spans, or evidentiary fidelity. |
| `summary` | General tasks where bounded prose is sufficient. |
| `latent_summary` | Compact state or derived memory where raw text should not be sent. |
| `decision_only` | Tasks that need the resulting decision or rule rather than the full discussion. |
| `citation_only` | Tasks that need a waypoint/reference but not content in the model context. |

The Work Order type, risk class, output shape, permission scope, quoteability,
token budget, and required fidelity SHOULD drive compression choice.

The governing optimization rule is:

[
oxed{
	ext{compress while marginal resource savings exceed expected information loss}
}
]

subject to the Work Order's fidelity threshold. Exact language, decisive
evidence, legally operative text, contradictory material, and audit-critical
source spans may require `raw_excerpt` even when a shorter representation is
available.

LLMLingua, LongLLMLingua, and Selective Context provide empirical evidence that
prompt compression can improve efficiency on evaluated workloads. They do not
establish universal compression ratios or justify discarding low-frequency
evidence. Goni MUST evaluate omission of decisive evidence, citation fidelity,
task quality, latency, and resource savings together.

A legal memo may therefore prefer `raw_excerpt`; a style-sensitive social
draft may prefer `summary` or `latent_summary`; a high-risk action may carry
`citation_only` audit refs while withholding sensitive content from the model
when the task can proceed without disclosure.
