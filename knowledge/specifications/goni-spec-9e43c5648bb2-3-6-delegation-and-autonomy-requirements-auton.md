---
id: GONI-SPEC-9E43C5648BB2
title: 3.6 Delegation and autonomy requirements (AUTON)
type: specification
status: draft
implementation_state: specified_only
proposition: '**AUTON-01 (autonomy corridors):** each task class (for example: email_reply, invoice_payment, calendar_change, doc_edit) must have an explicit corridor policy: no_go (never auto-execute), soft_gate (bounded execution + review), autopilot (auto-execute by default).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 3.6 Delegation and autonomy requirements (AUTON)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.6 Delegation and autonomy requirements (AUTON)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.6 Delegation and autonomy requirements (AUTON)

- **AUTON-01 (autonomy corridors):** each task class (for example:
  `email_reply`, `invoice_payment`, `calendar_change`, `doc_edit`) must have an
  explicit corridor policy:
  - `no_go` (never auto-execute),
  - `soft_gate` (bounded execution + review),
  - `autopilot` (auto-execute by default).
  For operator-facing UX and policy summaries, Goni may refer to these as three
  authority classes: `autopilot`, `soft_gate`, and `hard_gate`, where
  `hard_gate` means explicit human decision and includes the deny-only
  `no_go` floor.
- **AUTON-02 (risk-bounded default):** execution defaults to "auto unless risky."
  The system must compute a risk score per action and:
  - auto-execute when below corridor thresholds,
  - queue for review or escalate when thresholds are exceeded.
- **AUTON-03 (policy-level control):** meaningful human control is exercised at
  policy level (corridors, thresholds, allow/deny lists), not by requiring
  per-action confirmation for routine work.
- **AUTON-04 (offloading safeguards):** system must include:
  - anomaly-first review feed,
  - periodic post-hoc sampling of autonomous actions,
  - rapid downgrade/kill-switch controls for autonomy policies.
- **AUTON-05 (kernel-owned authority):** receipts, capability tokens, corridor
  policy, and durable memory semantics are kernel primitives. These may not be
  delegated to a third-party assistant framework, gateway, or extension host as
  the source of truth for authority, audit, or memory.
- **AUTON-06 (mediated external frameworks):** external assistant frameworks may
  provide UX surfaces, channel adapters, or optional mediated tool seats, but
  they must not own session authority, policy decisions, or durable memory
  provenance.
