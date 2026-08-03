---
id: GONI-SPEC-3C0787853B81
title: 1. Execution taxonomy (normative)
type: specification
status: draft
implementation_state: specified_only
proposition: 'Goni MUST classify each event into one execution layer: action: user-visible outcome (for example: prepare daily brief).'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-01-execution-metering.md
  heading: 1. Execution taxonomy (normative)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 1. Execution taxonomy (normative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Execution taxonomy (normative)
Goni MUST classify each event into one execution layer:
- `action`: user-visible outcome (for example: prepare daily brief).
- `tool`: side-effectful tool operation (for example: send email).
- `model`: inference operation (local or remote).
