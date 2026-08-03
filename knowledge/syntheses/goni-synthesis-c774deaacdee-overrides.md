---
id: GONI-SYNTHESIS-C774DEAACDEE
title: Overrides
type: synthesis
status: draft
implementation_state: specified_only
proposition: Prefer updating canonical contracts under blueprint/software/50-data/* before changing implementation.
domains:
- agent
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/meta/agents.software.template.md
  heading: Overrides
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Overrides

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Overrides
- Prefer updating canonical contracts under `blueprint/software/50-data/*` before changing implementation.
- Any change that affects data schemas, plane tags, or text confinement MUST update:
  - `blueprint/software/50-data/51-schemas-mvp.md` (contract)
  - `blueprint/software/50-data/53-schema-dsl-and-macros.md` (mechanism)
  - and the relevant kernel tests under `goni-prototype-lab:software/kernel/goni-schema/tests/`
