---
id: DELEG-01
title: DELEG-01 - Delegation and Autonomy
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: DELEG-01 Status: Specified only / roadmap This spec defines how Goni maximizes safe background execution of digital work while preserving policy-level human control.'
domains:
- specs
aliases:
- DELEGATION-AND-AUTONOMY
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: DELEG-01 - Delegation and Autonomy
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# DELEG-01 - Delegation and Autonomy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# DELEG-01 - Delegation and Autonomy
DOC-ID: DELEG-01
Status: Specified only / roadmap

This spec defines how Goni maximizes safe background execution of digital work
while preserving policy-level human control.

The pre-execution reconstruction layer is specified separately in
`DELEG-INT-01`. This document assumes that delegated execution already carries
an `interaction_mode`, Work Order, and Done Contract reference before corridor
policy is evaluated.

Delegation in Goni is not prompt relay. The system is expected to perform part
of the prompt-work on behalf of the user: infer missing structure, repair vague
intent into executable form, and acquire just enough extra context to act
safely. Those interpretive moves must remain visible, corrigible, and bounded
by policy rather than hidden inside model behavior
[[tomasev2026-intelligent-delegation]] [[zhang2025-ace]]
[[yang2025-contextagent]].
