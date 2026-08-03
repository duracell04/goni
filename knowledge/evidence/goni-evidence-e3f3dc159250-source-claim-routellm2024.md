---
id: GONI-EVIDENCE-E3F3DC159250
title: 'Source claim: routellm2024'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Routers trained from preference data can select between stronger and weaker LLMs to reduce cost while preserving much of strong-model performance.
domains:
- research
aliases: []
relations:
- type: supports
  target: EVID-ROUTE-01
sources:
- SRC-ROUTELLM2024
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[routellm2024]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: routellm2024

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[routellm2024]]
Claim: Routers trained from preference data can select between stronger and
weaker LLMs to reduce cost while preserving much of strong-model performance.
Relevance:
- Supports treating routing as a learnable control problem rather than a fixed
  "always best model" policy.
- Provides a future path for Goni Lab traces to train routing policies.
Used in:
- `blueprint/50-evidence/eval/EVID-ROUTE-01-frugal-sovereign-routing.md`
Source:
- https://sky.cs.berkeley.edu/project/routellm/
