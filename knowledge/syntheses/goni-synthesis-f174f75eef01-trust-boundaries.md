---
id: GONI-SYNTHESIS-F174F75EEF01
title: Trust boundaries
type: synthesis
status: draft
implementation_state: specified_only
proposition: The kernel is trusted for mediation and receipts.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/ARCHITECTURE.md
  heading: Trust boundaries
  revision: 0a497c0d5875633b0759b34fb5bd2aa6f9f0141c
---

# Trust boundaries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Trust boundaries
- The kernel is trusted for mediation and receipts.
- Tools and external text are untrusted.
- Egress is mediated by a gate.
