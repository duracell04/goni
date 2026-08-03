---
id: GONI-SPEC-920062F08248
title: 1. Enforcement objective
type: specification
status: draft
implementation_state: specified_only
proposition: 'Guarantee that untrusted tool/runtime components cannot bypass policy and receipt mediation for: network egress, filesystem mutation, external connector side effects.'
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-ENF-01-non-bypassable-mediation.md
  heading: 1. Enforcement objective
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 1. Enforcement objective

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Enforcement objective

Guarantee that untrusted tool/runtime components cannot bypass policy and
receipt mediation for:
- network egress,
- filesystem mutation,
- external connector side effects.
