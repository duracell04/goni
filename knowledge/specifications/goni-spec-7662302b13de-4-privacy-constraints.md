---
id: GONI-SPEC-7662302B13DE
title: 4. Privacy constraints
type: specification
status: draft
implementation_state: specified_only
proposition: Receipts MUST avoid raw transcript storage by default.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-02-receipt-metering-fields.md
  heading: 4. Privacy constraints
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 4. Privacy constraints

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Privacy constraints
- Receipts MUST avoid raw transcript storage by default.
- Metering fields MUST be computable from counters and hashes, not raw content.
