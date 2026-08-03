---
id: GONI-IMAP-2F6D70B5BB0F
title: 0.2 Practical architecture (rings + interrupts)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Goni OS treats agents as **local userland processes** and the LLM as a **rare, budgeted interrupt**, not a control loop.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 0.2 Practical architecture (rings + interrupts)
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 0.2 Practical architecture (rings + interrupts)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 0.2 Practical architecture (rings + interrupts)

Goni OS treats agents as **local userland processes** and the LLM as a **rare,
budgeted interrupt**, not a control loop. The practical architecture is a
three-ring model:
This structure is what makes Goni a "digital double": a local process that
observes, distills, and acts under explicit policy and receipts.

- **Ring 0 (Cognitive kernel):** observation bus, latent state store, policy
  engine, scheduler/interrupt controller.
- **Ring 1 (Always-on cognition):** encoders + predictor update latent state,
  compute surprisal/goal drift, and decide whether to raise interrupts.
- **Ring 2 (Userland):** agent runtime and solver/decoder services invoked on
  demand through kernel APIs.

Canonical specs:

- Latent state contract: `blueprint/30-specs/latent-state-contract.md`
- Agents and manifests: `blueprint/30-specs/agent-definition.md`, `blueprint/30-specs/agent-manifest.md`
- Tools and audit: `blueprint/30-specs/tool-capability-api.md`
- Scheduler/interrupts: `blueprint/30-specs/scheduler-and-interrupts.md`
- ITCR cascade: `blueprint/30-specs/itcr.md`

This section is a practical view; the formal planes \((\mathcal{A}, \mathcal{X},
\mathcal{K}, \mathcal{E})\) below define the invariants.
