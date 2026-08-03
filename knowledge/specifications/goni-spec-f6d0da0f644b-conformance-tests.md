---
id: GONI-SPEC-F6D0DA0F644B
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: Direct socket syscalls from tool runners fail in baseline policy profile.
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
  heading: Conformance tests
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- Direct socket syscalls from tool runners fail in baseline policy profile.
- Tool runners cannot write outside capability-authorized roots.
- All allowed egress events include trace and capability attribution fields.
- Forced policy-engine outage fails closed for effectful operations.
