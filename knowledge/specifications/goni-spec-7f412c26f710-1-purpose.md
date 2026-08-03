---
id: GONI-SPEC-7F412C26F710
title: 1. Purpose
type: specification
status: draft
implementation_state: specified_only
proposition: Goni must not treat every user message as a literal execution prompt.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 1. Purpose
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 1. Purpose

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Purpose

Goni must not treat every user message as a literal execution prompt. Before
execution, the runtime MUST:

- classify whether the task is delegated execution or co-creation,
- define what "done" means for the current turn,
- decide whether to assume, ask, propose candidate objectives, or block,
- decide whether the task requires audit-grade work,
- compile a stable Work Order that downstream policy and tools can audit.

This interface exists so prompt-work and audit-work become inspectable
control-plane state rather than hidden model behavior.
