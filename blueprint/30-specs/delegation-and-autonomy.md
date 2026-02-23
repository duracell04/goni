---
id: DELEG-01
type: SPEC
status: specified_only
---
# DELEG-01 - Delegation and Autonomy
DOC-ID: DELEG-01
Status: Specified only / roadmap

This spec defines how Goni maximizes safe background execution of digital work
while preserving policy-level human control.

## 1. Purpose

Goni is designed for policy-and-anomaly-first operation:

- users define constraints and priorities,
- the twin executes routine work end-to-end within those constraints,
- users review exceptions and policy drift, not every action.

This contract applies across mail, calendar, documents, payments, and other
tool-mediated surfaces.

## 2. Delegation model

### 2.1 Task classes

Every mediated action MUST be tagged with a `task_class` (for example:
`email_reply`, `invoice_payment`, `calendar_change`, `doc_edit`).

### 2.2 Autonomy corridors

Each `task_class` MUST declare one of:

- `no_go`: never execute automatically.
- `soft_gate`: execute only with bounded conditions and review gates.
- `autopilot`: execute by default without pre-approval.

Corridor assignment is policy data and versioned.

### 2.3 Risk dimensions

A normalized `risk_score` in `[0,1]` MUST be computed from, at minimum:

- reversibility and compensation path quality,
- blast radius (financial, legal, or reputational magnitude),
- ambiguity/uncertainty,
- policy sensitivity (regulated domain, restricted counterparty, or consent gate).

## 3. Default execution policy

Goni uses an "auto unless risky" policy:

- if corridor is `autopilot` and `risk_score <= theta_auto`, execute;
- if corridor is `soft_gate` and `risk_score <= theta_soft`, execute with
  queued review;
- otherwise escalate to user decision.

`theta_auto` and `theta_soft` are explicit policy parameters and must be auditable.

## 4. SOP lifecycle

Standard operating procedures (SOPs) are machine-executable templates.
Each SOP MUST move through:

1. `shadow` (suggestions only, no external side effects),
2. `approved` (user approved),
3. `autopilot` (eligible for automatic execution),
4. `revoked` (disabled and retained for audit).

Promotion to `autopilot` requires successful runs and no unresolved safety
findings over a configured observation window.

## 5. Oversight model

Oversight is post-hoc by default:

- anomaly-first feed (only blocked, high-risk, or policy-drift events),
- periodic sampling of autonomous actions,
- fast policy override and corridor downgrade.

This avoids per-action approval loops while preserving meaningful human control.

## 6. Autonomy packs and templates

Goni SHOULD support profile-based policy packs and SOP templates so users can
start with safe defaults and adapt over time.

Each imported pack MUST declare:

- covered task classes,
- default corridors and thresholds,
- declared no-go actions,
- provenance (pack version, author, policy hash).

## 7. Invariants

- **I1 - Policy primacy:** no autonomous execution without an explicit corridor
  and policy hash.
- **I2 - Risk-bounded autonomy:** autonomous execution requires computed risk
  below active thresholds.
- **I3 - Auditable delegation:** every autonomous or escalated action emits a
  receipt with autonomy and risk fields.
- **I4 - Fail closed:** if risk computation, policy load, or capability
  validation fails, execution is denied and logged.

## 8. Related specs

- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)

## 9. Upstream

- [Software requirements](/blueprint/software/10-requirements.md)
- [Software decisions](/blueprint/software/90-decisions.md)

## 10. Downstream

- [Conformance](/blueprint/software/30-conformance.md)
- [Eval lane](/blueprint/50-evidence/eval/README.md)

## 11. Adjacent

- [Metrics](/blueprint/docs/metrics.md)
- [Bibliography](/blueprint/docs/references/bibliography.md)

## Conformance tests

- TBD: add tests for this spec.
