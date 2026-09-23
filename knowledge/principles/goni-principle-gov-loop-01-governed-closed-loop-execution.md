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
sources: []
artifacts: []
uncertainty: "Specified systems framing. Individual loops may omit action-specific stages when a task is read-only or no durable state change is warranted."
legacy: []
---

# Governed Closed-Loop Execution

Goni is modeled as a governed closed-loop controller:

```text
Goal / State
-> Observe
-> Context
-> Cognition
-> Authorization
-> Action
-> Observe
-> Verify
-> Receipt
-> State Update
```

The authority step is a first-class boundary. Model output may propose an action;
it does not itself grant permission to execute that action.

The loop does not close at tool invocation. Where an action has external or
durable consequences, the system should observe the resulting state, verify
relevant postconditions, attach evidence to the action receipt, and update
canonical state only through the governing memory and policy contracts.

This framing makes later adaptation testable. Outcomes can be compared with
prior predictions, routing choices, uncertainty estimates, and verification
results without allowing those statistical signals to rewrite authority state
directly.
