# ARCHITECTURE

Status: specified only / roadmap

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

