---
id: GONI-SPEC-208C09984852
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: Metered executions emit receipts containing required execution fields.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-02-receipt-metering-fields.md
  heading: Conformance tests
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- Metered executions emit receipts containing required execution fields.
- `execution_type` values outside `action|tool|model` are rejected.
- Budget counters are non-negative and policy-checkable.
