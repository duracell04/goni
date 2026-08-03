---
id: GONI-EVIDENCE-B00A650DCCD4
title: 'Source claim: lin2026-agentic-harness-engineering'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Agent harnesses shape how models interact with repositories, tools, and execution environments; component, experience, and decision observability can make harness evolution explicit, evidence-backed, and revertible. Reported benchmark gains are promising but should be independently reproduced.
domains:
- research
aliases: []
relations:
- type: supports
  target: DOCTRINE-DELEG-01
- type: supports
  target: SYS-03
sources:
- SRC-LIN2026-AGENTIC-HARNESS-ENGINEERING
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[lin2026-agentic-harness-engineering]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: lin2026-agentic-harness-engineering

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[lin2026-agentic-harness-engineering]]
Claim: Agent harnesses shape how models interact with repositories, tools, and
execution environments; component, experience, and decision observability can
make harness evolution explicit, evidence-backed, and revertible. Reported
benchmark gains are promising but should be independently reproduced.
Relevance:
- Supports Goni's Harness Plane as a governed layer around model behavior.
- Motivates falsifiable harness changes with predictions, eval windows,
  receipt-backed evidence, and rollback conditions.
Used in:
- `blueprint/10-product/15-delegation-doctrine.md` (Harness Governance)
- `blueprint/20-system/50-learning-loop.md` (Harness observability)
Source:
- https://arxiv.org/abs/2604.25850
