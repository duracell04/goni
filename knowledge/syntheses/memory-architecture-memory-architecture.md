---
id: MEMORY-ARCHITECTURE
title: Memory Architecture
type: synthesis
status: draft
implementation_state: specified_only
proposition: "\uFEFF# Goni Memory Architecture (Legacy Draft) **Status:** Legacy UX concept (kept for historical context)."
domains:
- memory
aliases: []
relations:
- type: synthesizes
  target: GONI-SYNTHESIS-AE048AF3026D
- type: synthesizes
  target: GONI-SYNTHESIS-161FC810382A
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/memory-architecture.md
  heading: Memory Architecture
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# Memory Architecture

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

﻿# Goni Memory Architecture (Legacy Draft)

> **Status:** Legacy UX concept (kept for historical context).
>
> **Source of truth for current architecture:**
> - Kernel-level latent state: `blueprint/30-specs/latent-state-contract.md` (LSC-01)
> - Plane model and payload types: `blueprint/software/50-data/10-axioms-and-planes.md`
> - MVP schema tables: `blueprint/software/50-data/51-schemas-mvp.md`
>
> This document predates the Latent State Contract and uses an older, UX-first framing
> (working/episodic/semantic/procedural). It is useful for product thinking, but it is not
> the normative contract for implementation.

This note captures the **hard** and **soft** commitments for Goni's long-term memory so it strengthens users rather than replacing their thinking.
