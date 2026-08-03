---
id: GONI-PRINCIPLE-2A48B2D4DA86
title: Core principle
type: principle
status: draft
implementation_state: specified_only
proposition: Goni must remain modular, sovereign, and independently governable.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/ARCHITECTURE.md
  heading: Core principle
  revision: 0a497c0d5875633b0759b34fb5bd2aa6f9f0141c
---

# Core principle

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
