---
id: ESCALATION-HANDOVER-01
title: Escalation state-transfer contract
type: specification
status: draft
implementation_state: specified_only
proposition: When Goni escalates an anticipated or delegated workflow to the principal, the escalation should transfer sufficient state for effective takeover, including what happened, what was expected, where reality diverged, actions already taken, current state, evidence, available options, and the unresolved decision.
domains:
- specs
- delegation
- hci
aliases:
- takeover-package
- escalation-state-transfer
relations:
- type: depends_on
  target: ANTICIPATORY-DELEGATION-01
sources:
- SRC-BAINBRIDGE1983-IRONIES-AUTOMATION
- SRC-LEE2004-TRUST-AUTOMATION
artifacts: []
uncertainty: The minimum sufficient handover payload and presentation format require user testing across task classes and consequence levels.
legacy: []
---

# Escalation state-transfer contract

Escalation is a transfer of operational control, not merely a notification.

When Goni reaches a policy, uncertainty, anomaly, or authority boundary that
requires principal intervention, the escalation should provide a compact
handover package containing stable references or bounded summaries of:

- the current objective and WorkOrder,
- the triggering event or observation,
- the state Goni expected,
- the observed divergence or exception,
- actions already prepared or executed,
- the current external and internal state relevant to the decision,
- evidence and uncertainty,
- available options and their material consequences,
- the exact unresolved decision or authority requirement,
- rollback or compensation options when applicable.

The package should preserve situation awareness while minimizing interruption
cost. It should enable the principal to retake control without reconstructing
the workflow from raw logs.
