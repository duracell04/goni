---
id: OPSOV-OBJ-01
title: "Local hosting is insufficient as a sovereignty test"
type: objection
status: draft
implementation_state: not_applicable
proposition: "Local inference, private keys and local data alone cannot establish firmware integrity, privileged control, update trustworthiness or disaster recoverability."
domains:
- security
- research
aliases: []
relations:
- type: objects_to
  target: GONI-SYNTHESIS-8577DD9A5AD1
- type: supports
  target: OPSOV-PRINC-01
- type: derived_from
  target: OPSOV-EVID-SECURITY-01
sources:
- SRC-NIST-SP800-193
- SRC-NIST-SP800-161R1-UPD1
- SRC-NIST-SP800-160V2R1
artifacts: []
uncertainty: "A bounded objection to sufficient proof, not a rejection of local-first designs."
legacy: []
---

# Local hosting is insufficient as a sovereignty test

The existing core-posture synthesis characterizes sovereign deployment through local inference, canonical local authority and control of keys, weights, policy and data. These are valuable goals, but privileged firmware, software-update channels, recovery procedures and external actors may still subvert effective control.

A compromised enforcement component can falsify both authorization and local receipts. Owner-held hardware can fail, and owner-held keys can be lost. Patch and backup duties introduce measurable costs. Sovereignty assurance consequently requires a specified threat model, privileged-dependency inventory and independent verification.

This node identifies a limitation of treating locality as a *sufficient* condition.
