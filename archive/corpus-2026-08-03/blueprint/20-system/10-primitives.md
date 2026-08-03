# Goni Primitives

This page defines the core product primitives in one place so terms stay concrete.

## 1) Daily Brief
- Purpose: low-interruption summary of what changed and what needs attention.
- Contract anchor: `schemas/cards/daily_brief.schema.json`.
- Core invariant: no side effects; every item links to evidence or receipt.
- Metrics: brief value rate, false-interruption rate, time-to-scan.

## 2) Action Card
- Purpose: explicit propose-before-act unit for side-effectful work.
- Contract anchor: `schemas/cards/action_card.schema.json`.
- Core invariant: action requires visible intent, required capabilities, and status transitions.
- Metrics: TTFA, approve/edit/reject rates, undo rate, execution success.

## 3) Receipt
- Purpose: immutable audit record for mediated actions.
- Contract anchors: `30-specs/receipts.md`, `schemas/receipts/receipt.schema.json`.
- Core invariant: every mediated action emits one verifiable receipt.
- Metrics: receipt coverage, verification latency, redaction coverage.

## 4) Vault
- Purpose: local system of record for durable memory and provenance pointers.
- Contract anchors: `software/vault/README.md`, `30-specs/sync-policies.md`.
- Core invariant: long-term memory is local-first; cloud paths use minimized context.
- Metrics: retrieval p95, ingest latency, citation coverage.

## 5) Council
- Purpose: policy-gated escalation path for high-risk, high-uncertainty, or long-context tasks.
- Contract anchors: `docs/llm-council.md`, `docs/remote-llm-architecture.md`.
- Core invariant: council is optional and never bypasses policy, budgets, or network gate.
- Metrics: escalation rate, added latency, and resource overhead per escalated execution.

## 6) Gates and Scheduling
- Purpose: non-bypassable mediation and predictable QoS.
- Contract anchors: `30-specs/tool-capability-api.md`, `30-specs/network-gate-and-anonymity.md`, `30-specs/scheduler-and-interrupts.md`.
- Core invariant: no ambient authority; interactive work preempts background work.
- Metrics: preemption latency, cancel-to-quiescent, blocked egress rate.

