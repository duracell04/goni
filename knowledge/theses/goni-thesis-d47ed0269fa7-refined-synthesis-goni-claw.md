---
id: GONI-THESIS-D47ED0269FA7
title: 'Refined synthesis: "Goni Claw"'
type: thesis
status: draft
implementation_state: specified_only
proposition: 'The useful synthesis is not "base Goni on OpenClaw." It is: Goni owns the kernel, policy, receipts, corridors, and memory spine.'
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/competitors/openclaw.md
  heading: 'Refined synthesis: "Goni Claw"'
  revision: 2dc57be8861d183c1d31793c92259fc63f22d1ad
---

# Refined synthesis: "Goni Claw"

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Refined synthesis: "Goni Claw"

The useful synthesis is not "base Goni on OpenClaw." It is:

- Goni owns the kernel, policy, receipts, corridors, and memory spine.
- An OpenClaw-like layer provides the operator front door:
  channel routing, action surfaces, integrations, and extension UX.

In that refined model, "Goni Claw" means:

- OpenClaw-style interaction model,
- Goni-owned trust model.

Practical rule:

- steal the surface ideas,
- do not outsource the control plane.

If implemented, the OpenClaw-like layer must sit above kernel mediation as a
gateway or adapter seat. It cannot become the source of truth for session
authority, tool approval, or audit semantics.
