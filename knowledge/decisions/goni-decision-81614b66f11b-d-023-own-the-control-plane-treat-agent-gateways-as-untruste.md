---
id: GONI-DECISION-81614B66F11B
title: D-023 - Own the control plane; treat agent gateways as untrusted seats
type: decision
status: draft
implementation_state: specified_only
proposition: '**Status:** Accepted **Date:** 2026-04-16 **Formal statement** Let K denote the sovereign Goni kernel.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-023 - Own the control plane; treat agent gateways as untrusted seats
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-023 - Own the control plane; treat agent gateways as untrusted seats

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-023 - Own the control plane; treat agent gateways as untrusted seats

**Status:** Accepted
**Date:** 2026-04-16

**Formal statement**

Let `K` denote the sovereign Goni kernel. The following primitives are
kernel-owned and non-outsourcable:

- receipt emission and verification,
- capability-token issuance and enforcement,
- authority corridors (`autopilot` / `soft_gate` / `hard_gate` as the
  operator-facing classes, with `no_go` retained as the deny-only policy
  floor in current specs),
- durable memory spine semantics and provenance.

Any third-party agent gateway or assistant framework `G` may be integrated only
as:

- a channel adapter,
- an optional mediated tool or gateway seat,
- or a UX inspiration/reference.

`G` MUST NOT replace `K` as the source of truth for session authority, policy,
effect mediation, or durable memory semantics.

**Rationale**

- Sovereignty requires control over the layer that governs assistant behavior,
  not merely local hosting of someone else's framework.
- A self-hosted dependency with its own session model, tool registry, or audit
  semantics is still an external control plane.
- Trust lives in the kernel boundary: why an action was allowed, how it can be
  replayed, and whether it can be rolled back must remain legible inside Goni.

**Consequence**

- Faster time-to-market via OpenClaw-style reuse is explicitly rejected as the
  sovereign architecture path.
- External assistant frameworks may accelerate experiments, but only behind
  kernel mediation and never as the sovereign kernel.
- The minimum sovereign kernel is fixed:
  - receipt log for every mediated action,
  - capability token before every effectful execution,
  - kernel-owned authority corridors,
  - kernel-owned memory spine.
- Requirements and specs must treat these primitives as non-delegable kernel
  responsibilities.
