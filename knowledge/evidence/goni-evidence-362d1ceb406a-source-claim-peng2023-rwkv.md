---
id: GONI-EVIDENCE-362D1CEB406A
title: 'Source claim: peng2023-rwkv'
type: evidence
status: draft
implementation_state: not_applicable
proposition: RWKV combines transformer-style training parallelism with recurrent inference and reports scaling to large model sizes.
domains:
- research
aliases: []
relations:
- type: supports
  target: SYS-02
sources:
- SRC-PENG2023-RWKV
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[peng2023-rwkv]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: peng2023-rwkv

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[peng2023-rwkv]]
Claim: RWKV combines transformer-style training parallelism with recurrent
inference and reports scaling to large model sizes.
Relevance:
- Evidence that architecture claims should be evaluated at practical scale.
Used in:
- `blueprint/20-system/30-performance.md` (tokenization and scale realism)
Source:
- https://arxiv.org/abs/2305.13048
