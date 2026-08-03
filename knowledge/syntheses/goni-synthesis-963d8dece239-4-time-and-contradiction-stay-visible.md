---
id: GONI-SYNTHESIS-963D8DECE239
title: 4. Time and contradiction stay visible
type: synthesis
status: draft
implementation_state: specified_only
proposition: Knowledge changes over time.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/65-local-sovereign-knowledge-runtime.md
  heading: 4. Time and contradiction stay visible
  revision: 43414875152ae18f9977f21c9786b2d7025081ac
---

# 4. Time and contradiction stay visible

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Time and contradiction stay visible

Knowledge changes over time. The runtime distinguishes when an event occurred,
when it was recorded, when a claim or rule applied, and when it was superseded
or ceased to apply. As-of queries use the requested validity window rather than
silently applying the newest available text to the past.

Contradiction is first-class data, not a database defect. It can reveal factual
disagreement, historical change, competing scope or jurisdiction, a category
mistake, or divergence between formal policy and actual practice.

Query behavior follows the owner's purpose:

- **Descriptive:** return the material competing claims and their provenance.
- **Historical:** return what applied during the requested time window and
  identify later changes separately.
- **Operational:** apply a controlling rule only when the principal or an
  explicitly delegated role supplied one; preserve material dissent and
  conflicting practice as context.

When no controlling rule exists, the runtime surfaces the conflict or asks for
an authority decision. It does not manufacture consensus. Resolving a conflict
may change operational selection, but it does not erase the losing claim or
its provenance.
