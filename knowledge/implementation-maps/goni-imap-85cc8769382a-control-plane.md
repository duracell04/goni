---
id: GONI-IMAP-85CC8769382A
title: Control Plane
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '[ ] The scheduler uses a MaxWeight-like rule with a Lyapunov function \(L\) as documented.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: Control Plane
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# Control Plane

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Control Plane

- [ ] The scheduler uses a MaxWeight-like rule with a Lyapunov function \(L\) as documented.  
- [ ] Simulated workloads with load below \(\alpha\) show bounded queues (no drift to infinity).  
- [ ] The router is evaluated on labelled data and achieves average regret below a configured threshold (MVP: = 0.1).
- [ ] Delegated actions satisfy K3 (corridor + risk threshold required for autonomous commit).
- [ ] Escalation precision/recall are measured on labelled traces.
- [ ] Autonomous execution rate and unsafe autonomy incident rate are reported.
