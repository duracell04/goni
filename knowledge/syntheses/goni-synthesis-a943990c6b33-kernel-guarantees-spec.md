---
id: GONI-SYNTHESIS-A943990C6B33
title: Kernel guarantees (spec)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Target guarantees; see goni-prototype-lab:goni-lab/STATUS.md for implementation status.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: Kernel guarantees (spec)
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# Kernel guarantees (spec)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Kernel guarantees (spec)

Target guarantees; see goni-prototype-lab:goni-lab/STATUS.md for implementation status.

- Tool side effects are intended to be mediated by the kernel boundary.
- Network egress is intended to go through the egress gate.
- Every mediated action is intended to produce a receipt.
- Budgets are intended to be enforced at mediation boundaries.
- Receipts are intended to be minimal by default.

**Trusted computing base (TCB)**

- Kernel mediation and receipt components.
- Egress gate.
- Sovereignty means owning this control plane, not merely self-hosting a
  third-party assistant framework.
