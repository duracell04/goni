# ARCHITECTURE

Status: Specified only / roadmap

This document summarizes the current architecture and trust boundaries.
It is a short index; the formal contracts live in blueprint/30-specs/ and blueprint/software/.

## Core principle

Goni must remain modular, sovereign, and independently governable.

Every external dependency must be replaceable; every governance decision must
remain local, inspectable, and receipted. Third-party tools may provide
orchestration, inference, retrieval, UIs, voice, workflow automation, model
gateways, and hardware acceleration, but they must not own Goni's kernel
contracts, memory governance, policy, approvals, receipts, identity, or
rollback semantics.

Canonical formulation:

- Interfaces before implementations.
- No substrate owns canonical state.
- Local authority beats remote convenience.
- Portability is a design requirement.
- Promotion into the core is slow and evidence-gated.

See [Software Architecture](/blueprint/software/20-architecture.md) for the
formal architecture placement.

## Cognitive exocortex framing

The [Cognitive Exocortex Model](/blueprint/20-system/60-cognitive-exocortex-model.md)
provides a non-normative way to explain how Goni surrounds the human operator:

- the human first brain retains goals, values, delegated authority, correction,
  and veto;
- the second brain is governed durable memory over existing Knowledge and
  Memory responsibilities;
- the third brain is active cognition across the Harness, Context, Control, and
  Execution responsibilities;
- digital meninges group the cross-cutting security, orchestration,
  provenance, and epistemic controls.

This framing adds no service, schema, authority, trust boundary, or competing
contract. The formal architecture and canonical specifications remain the
source of truth.

## Components
- Orchestrator (kernel HTTP): request intake and scheduling.
- LLM runtime adapter: local inference engine integration.
- Retrieval: vector/sparse/graph backends and context assembly.
- Policy and receipts: capability enforcement and audit trail.

## Dataflow (high level)
Event -> scheduler -> model/tool -> receipt -> vault

## Trust boundaries
- The kernel is trusted for mediation and receipts.
- Tools and external text are untrusted.
- Egress is mediated by a gate.

