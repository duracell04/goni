---
id: GONI-SPEC-5ECB575FBFBC
title: 6. Synthetic input and tools
type: specification
status: draft
implementation_state: specified_only
proposition: Synthetic input is a tool syscall.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 6. Synthetic input and tools
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 6. Synthetic input and tools

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Synthetic input and tools

Synthetic input is a tool syscall. Mouse, keyboard, scroll, drag, browser DOM
mutation, shell command, filesystem write, external API mutation, and publish
actions are actuation events. They require:

- a Work Order and Done Contract when delegated,
- a capability token with an actuation grant,
- a sandbox profile that meets or exceeds the action class,
- autonomy corridor evaluation,
- approval evidence when required,
- idempotency and rollback/repair metadata where possible,
- a canonical Goni receipt.

External assistant logs, browser extension logs, operating-system event logs, or
third-party computer-use traces do not replace Goni receipts.
