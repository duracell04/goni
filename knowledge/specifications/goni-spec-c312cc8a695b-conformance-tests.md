---
id: GONI-SPEC-C312CC8A695B
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: Execution records include one valid execution_type.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-01-execution-metering.md
  heading: Conformance tests
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- Execution records include one valid `execution_type`.
- Metering counters are non-negative and present when applicable.
- Retries preserve parent-child execution linkage.
- Hard budgets block additional remote calls.
