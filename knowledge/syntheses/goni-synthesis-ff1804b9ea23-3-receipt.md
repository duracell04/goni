---
id: GONI-SYNTHESIS-FF1804B9EA23
title: 3) Receipt
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Purpose: immutable audit record for mediated actions.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/10-primitives.md
  heading: 3) Receipt
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3) Receipt

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3) Receipt
- Purpose: immutable audit record for mediated actions.
- Contract anchors: `30-specs/receipts.md`, `schemas/receipts/receipt.schema.json`.
- Core invariant: every mediated action emits one verifiable receipt.
- Metrics: receipt coverage, verification latency, redaction coverage.
