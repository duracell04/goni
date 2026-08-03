---
id: GONI-EXPERIMENT-5F299100AA67
title: Benchmark shape
type: experiment
status: draft
implementation_state: not_applicable
proposition: replay fixed traces that include screen observations, OCR/accessibility extraction, memory writes, remote model submission, and synthetic input proposals run each trace under policy bundles that grant only one power, selected pairs of powers, and the full governed chain compare allowed, denied, review, and escalated outcomes against labels
domains:
- agent
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-DESK-01-desktop-agent-firewall.md
  heading: Benchmark shape
  revision: aac7c2d833bd8db8894cb18deb97d6bc13e0b7b3
---

# Benchmark shape

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Benchmark shape

- replay fixed traces that include screen observations, OCR/accessibility
  extraction, memory writes, remote model submission, and synthetic input
  proposals
- run each trace under policy bundles that grant only one power, selected
  pairs of powers, and the full governed chain
- compare allowed, denied, review, and escalated outcomes against labels
- verify receipts and audit records contain `boundary_basis`, policy hash,
  Work Order refs for delegated actions, and no raw private content by default
