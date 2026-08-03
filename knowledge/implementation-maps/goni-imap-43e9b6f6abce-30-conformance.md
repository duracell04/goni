---
id: GONI-IMAP-43E9B6F6ABCE
title: 30 Conformance
type: implementation-map
status: draft
implementation_state: specified_only
proposition: "\uFEFF# 30 – Conformance Criteria & Proof Obligations (MVP Node) Status: v0.1 (2025-11-30) Scope: Single-node Goni kernel (no clustering) Audience: Researchers, systems engineers, auditors Normative contracts referenced by this document: Latent State Contract (LSC-01): blueprint/30-specs/latent-state-contract.md Delegation interface (DELEG-INT-01): blueprint/30-specs/delegation-interface.md"
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: 30 Conformance
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# 30 Conformance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

﻿# 30 – Conformance Criteria & Proof Obligations (MVP Node)

Status: v0.1 (2025-11-30)
Scope: Single-node Goni kernel (no clustering)
Audience: Researchers, systems engineers, auditors

Normative contracts referenced by this document:

- Latent State Contract (LSC-01): `blueprint/30-specs/latent-state-contract.md`
- Delegation interface (DELEG-INT-01): `blueprint/30-specs/delegation-interface.md`
- Tool syscall envelope (TOOL-01): `blueprint/30-specs/tool-capability-api.md`
- Vision, memory, and actuation boundaries (BOUND-01):
  `blueprint/30-specs/vision-memory-actuation-boundaries.md`
- Agent/process model: `blueprint/30-specs/agent-definition.md`, `blueprint/30-specs/agent-manifest.md`
- Scheduler/interrupt semantics: `blueprint/30-specs/scheduler-and-interrupts.md`
- Delegation/autonomy semantics: `blueprint/30-specs/delegation-and-autonomy.md`

> A **conformant Goni node** is one whose implementation can be mapped onto the four-plane model  
> $$
> N = (\mathcal{A}, \mathcal{X}, \mathcal{K}, \mathcal{E})
> $$
> and that satisfies the invariants and proof obligations listed below, at least in an MVP / empirical sense.

---
