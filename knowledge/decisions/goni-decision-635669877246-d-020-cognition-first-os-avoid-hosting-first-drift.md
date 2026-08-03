---
id: GONI-DECISION-635669877246
title: D-020 - Cognition-first OS; avoid hosting-first drift
type: decision
status: draft
implementation_state: specified_only
proposition: '**Status:** Accepted **Date:** 2025-12-30 **Formal statement** Goni is a cognition-first OS.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-020 - Cognition-first OS; avoid hosting-first drift
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-020 - Cognition-first OS; avoid hosting-first drift

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-020 - Cognition-first OS; avoid hosting-first drift

**Status:** Accepted
**Date:** 2025-12-30

**Formal statement**

Goni is a cognition-first OS. Formally, the core product is defined by:

- identity and policy mediation of agents and tools,
- capability-gated syscalls with audit attribution,
- local-first execution with optional remote presence.

Hosting or general-purpose app platform features are outside the kernel contract
and must not be required for correctness of \(\mathsf{Run}\).

**Rationale**

- Reference product patterns show users want a complete system, but the Goni wedge
  is cognition-first, not hosting-first.
- Prevents scope drift into "self-hosted cloud distro" while still allowing a
  focused ecosystem of agents and tools.

**Consequence**

- We ship identity, policy, agent runtime, and remote presence before any general
  hosting platform features.
- Marketplace/install flows are scoped to agents and tools, not arbitrary services.
- Any proposal to add hosting-first features must show it does not undermine the
  cognition-first contract or local-first correctness.

---
