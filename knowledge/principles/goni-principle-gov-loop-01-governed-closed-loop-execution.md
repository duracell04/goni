---
id: GONI-PRINCIPLE-GOV-LOOP-01
title: "Governed Closed-Loop Execution"
type: principle
status: draft
implementation_state: specified_only
proposition: "Goni should execute delegated work as a governed closed loop in which cognition proposes, the kernel authorizes, tools act, external consequences are observed, verification produces evidence, receipts record the boundary decision, and canonical state is updated."
domains:
- agent
- kernel
- system
aliases: []
relations:
- type: refines
  target: DOCTRINE-DELEG-01
- type: depends_on
  target: TOOL-01
- type: depends_on
  target: REC-01
sources:
- SRC-TOMASEV2026-INTELLIGENT-DELEGATION
artifacts: []
uncertainty: "Specified systems framing. Individual loops may omit steps when the task is read-only or no durable state change is warranted."
legacy: []
---

# Governed Closed-Loop Execution

Goni is modeled as a governed closed-loop controller:

[
Goal/State ightarrow Observe ightarrow Context ightarrow Cognition ightarrow Authorization ightarrow Action ightarrow Observe ightarrow Verify ightarrow Receipt ightarrow State Update
]

The authority step is a first-class boundary. Model output may propose an action; it does not itself grant permission to execute that action.

The loop closes only after external consequences are observed and the relevant evidence, receipt, and canonical state are updated.
