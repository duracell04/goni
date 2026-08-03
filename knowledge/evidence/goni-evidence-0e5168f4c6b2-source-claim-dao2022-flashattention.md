---
id: GONI-EVIDENCE-0E5168F4C6B2
title: 'Source claim: dao2022-flashattention'
type: evidence
status: draft
implementation_state: not_applicable
proposition: IO-aware exact attention implementation provides substantial practical speed/memory gains without changing attention semantics.
domains:
- research
aliases: []
relations:
- type: supports
  target: SYS-02
sources:
- SRC-DAO2022-FLASHATTENTION
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[dao2022-flashattention]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: dao2022-flashattention

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[dao2022-flashattention]]
Claim: IO-aware exact attention implementation provides substantial practical
speed/memory gains without changing attention semantics.
Relevance:
- Shows wall-clock can dominate asymptotic arguments in practical regimes.
Used in:
- `blueprint/20-system/30-performance.md` (big-O vs wall-clock section)
Source:
- https://arxiv.org/abs/2205.14135
