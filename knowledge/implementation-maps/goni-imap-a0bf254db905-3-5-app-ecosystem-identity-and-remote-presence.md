---
id: GONI-IMAP-A0BF254DB905
title: 3.5 App ecosystem, identity, and remote presence
type: implementation-map
status: draft
implementation_state: specified_only
proposition: We treat product completeness as part of the architecture, not a UI afterthought.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 3.5 App ecosystem, identity, and remote presence
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 3.5 App ecosystem, identity, and remote presence

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.5 App ecosystem, identity, and remote presence

We treat product completeness as part of the architecture, not a UI afterthought.
The Control and Execution planes expose explicit slots for identity, packaging,
and remote access:

- **Identity plane (logical):** user identity, agent identity, capability issuance,
  and audit attribution. This binds UI sessions to agent actions and logs.
- **Marketplace/install flow:** signed agent packages, manifest validation, policy
  prompts, and budget enforcement before activation.
- **Remote presence:** secure tunnels are modeled as capability-gated tools; there
  is no implicit "open port" path. Remote access is revocable and logged.

This section is a structural requirement derived from reference product patterns
(see `blueprint/docs/reference-products/olares.md`).


---
