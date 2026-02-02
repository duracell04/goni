# ARCHITECTURE

Status: specified only / roadmap

This document summarizes the current architecture and trust boundaries.
It is a short index; the formal contracts live in blueprint/docs/specs/ and blueprint/software/.

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
