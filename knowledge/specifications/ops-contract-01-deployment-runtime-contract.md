---
id: OPS-CONTRACT-01
title: "Deployment-to-runtime operational contracts"
type: specification
status: draft
implementation_state: specified_only
proposition: "Deployment lifecycle events must preserve kernel-owned authoritative state, auditable effects, and safe recovery semantics."
domains:
- software
- kernel
- delivery
aliases: []
relations:
- type: refines
  target: GONI-SPEC-14AA0EDAFDD4
- type: depends_on
  target: GONI-SPEC-4D10349C481A
- type: depends_on
  target: GONI-PRINCIPLE-3CE61BB5417A
sources:
- SRC-MASTOURI2026-AGENT-OPS
artifacts: []
uncertainty: "Proposed cross-boundary operational requirements, not an implementation or verified durability guarantee."
legacy: []
---

# Deployment-to-runtime operational contracts

This specification extends existing transactional-tool and idempotency rules to the deployment boundary, without prescribing a database or orchestrator.

## Contract

1. **State ownership.** Identify authoritative stores for mandates, policy/capability decisions, approvals, work orders, execution intents, receipts, revocations, and pending actions. Label caches and search indexes as rebuildable only where reconstruction is demonstrated.
2. **Durable intent.** Before an external consequential effect is attempted, commit an authorized intent with a stable idempotency reference. If the durable intent cannot be written, do not execute the effect.
3. **Outcome evidence.** Persist the observed outcome as success, denial, failure, or uncertainty. A failed receipt write MUST NOT be silently counted as success. If an effect may have occurred, preserve an uncertain status for reconciliation.
4. **Recovery.** Preserve authoritative state during restart, container replacement, interrupted approvals, graceful shutdown, and dependency failure, or fail safely. Do not blindly repeat external effects with uncertain outcomes. Use provider state and idempotency guarantees where available, with human escalation when verification is impossible.
5. **Atomicity.** Local state/receipt atomicity extends only to the declared kernel-controlled commit boundary. Email delivery, payment, and third-party changes may require verification or compensation rather than rollback.
6. **Deployment agreement.** Release configuration MUST state persistent mounts, service and network dependencies, secrets, schema compatibility, migration, backup/restore, and an achievable recovery or rollback target.

## Required verification

Map each authoritative data surface to its actual deployment resource. Use fault injection for crashed processes, replaced containers, unavailable stores, failed receipt writes, interrupted effects, and uncertain external responses. Syntax checks and service-health responses alone cannot establish these contracts.
