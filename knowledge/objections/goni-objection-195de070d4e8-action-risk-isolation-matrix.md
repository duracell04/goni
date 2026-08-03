---
id: GONI-OBJECTION-195DE070D4E8
title: Action-risk isolation matrix
type: objection
status: draft
implementation_state: not_applicable
proposition: Sandboxing is a hard trust boundary, not a deployment preference.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/isolation-and-tool-sandboxes.md
  heading: Action-risk isolation matrix
  revision: 9a29f6eb9fee912e41d8e4c7aa0b325aff6cf7b2
---

# Action-risk isolation matrix

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Action-risk isolation matrix

Sandboxing is a hard trust boundary, not a deployment preference. Minimum
isolation follows the action class:

| Action class | Examples | Minimum isolation | Gate type |
| --- | --- | --- | --- |
| `read_only_retrieval` | local search, metadata read, vector lookup | process isolation or equivalent capability sandbox | capability check |
| `screen_capture_observation` | screen/window/tab capture, accessibility-tree read, OCR candidate extraction | OS permission boundary plus isolated capture/extraction worker; no memory write or network by default | observation/extraction capability check |
| `desktop_browser_actuation` | synthetic mouse/keyboard/scroll, browser DOM mutation, GUI automation | isolated desktop/browser session, container, or microVM-class boundary for untrusted or internet-exposed surfaces; no host credentials by default | soft or hard gate by risk |
| `reversible_local_write` | draft file, temporary index update, local note edit with snapshot | container/gVisor-style sandbox plus rollback reference | soft gate or autopilot if policy allows |
| `code_execution` | shell, Python, browser automation with local credentials, package install | microVM/Firecracker-class isolation or stronger; no ambient host credentials | hard gate unless explicitly pre-approved |
| `external_side_effect` | email send, calendar mutation, API write, publish action | isolated executor plus egress gate and idempotency/rollback plan where possible | soft or hard gate by risk |
| `irreversible_high_risk` | financial, legal, account deletion, regulated data release | strongest available isolation, dual receipt, explicit human confirmation | hard gate |

The matrix defines minimums. Policy may require stronger isolation or block the
action. If the required isolation is unavailable, execution fails closed.

Screen capture, accessibility extraction, and synthetic input do not share one
authority class. Observation/extraction workers must not inherit actuation or
memory-write authority. Desktop or browser actuation must present an actuation
grant and pass BOUND-01 before tool execution.
