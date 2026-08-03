---
id: GONI-SYNTHESIS-75F0886EDC60
title: Overrides
type: synthesis
status: draft
implementation_state: specified_only
proposition: Kernel crates must respect plane boundaries (Data / Context / Control / Execution).
domains:
- agent
- kernel
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/meta/agents.kernel.template.md
  heading: Overrides
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Overrides

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Overrides
- Kernel crates must respect plane boundaries (Data / Context / Control / Execution). Do not bypass contracts for convenience.
- Any change that impacts routing, scheduling, or tool execution MUST update the corresponding spec:
  - routing/scheduling: `blueprint/30-specs/scheduler-and-interrupts.md`
  - tool execution/permissions: `blueprint/30-specs/tool-capability-api.md`
