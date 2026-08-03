---
id: GONI-EVIDENCE-EB30DE3D1AB5
title: 'Source claim: kwon2023-vllm'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Identifies KV-cache memory management as central to LLM serving throughput and introduces PagedAttention to reduce fragmentation.
domains:
- research
aliases: []
relations:
- type: supports
  target: GONI-SYNTHESIS-8A0BE005D5C2
sources:
- SRC-KWON2023-VLLM
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[kwon2023-vllm]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: kwon2023-vllm

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[kwon2023-vllm]]
Claim: Identifies KV-cache memory management as central to LLM serving
throughput and introduces PagedAttention to reduce fragmentation.
Relevance:
- Justifies treating KV-cache residency as a first-class scheduling concern.
Used in:
- `blueprint/20-system/40-agentic-kernel-foundations.md` (Scheduling)
Source:
- https://arxiv.org/pdf/2309.06180
