---
id: GONI-SPEC-DB77336F8DE8
title: 4. Canonical Flow
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every delegated robot action follows this logical flow: The flow may stop early when policy denies a transition, when approval is required, when sensor evidence is insufficient, when a safety envelope is violated, when the mandate is revoked, or when the robot adapter cannot provide required receipt evidence.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 4. Canonical Flow
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 4. Canonical Flow

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Canonical Flow

Every delegated robot action follows this logical flow:

```text
Human intent
-> Work Order / Done Contract
-> Robot Mandate
-> Environment Scope
-> Safety Envelope
-> Skill / Tool Call
-> Local Policy Check
-> Robot Execution
-> Verification
-> Receipt
-> Memory Update
```

The flow may stop early when policy denies a transition, when approval is
required, when sensor evidence is insufficient, when a safety envelope is
violated, when the mandate is revoked, or when the robot adapter cannot provide
required receipt evidence. A stopped flow still emits an auditable denial,
escalation, or no-op record.

Robot actions are tool-mediated side effects. They must preserve the
DELEG-INT-01 Work Order, Done Contract, autonomy mode, risk basis, capability
token, boundary basis, and receipt chain required by TOOL-01 and REC-01.
