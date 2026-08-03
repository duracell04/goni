---
id: GONI-DECISION-4042B98600BD
title: D-018 - Latent-first cognition as an architectural stance
type: decision
status: draft
implementation_state: specified_only
proposition: '**Status:** Proposed **Date:** YYYY-MM-DD **Formal statement** We adopt **latent-first cognition** as a guiding stance at the architecture/interface level: Maintain a latent "world state" as the primary internal representation.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-018 - Latent-first cognition as an architectural stance
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-018 - Latent-first cognition as an architectural stance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-018 - Latent-first cognition as an architectural stance

**Status:** Proposed  
**Date:** YYYY-MM-DD

**Formal statement**

We adopt **latent-first cognition** as a guiding stance at the architecture/interface level:

- Maintain a latent "world state" as the primary internal representation.  
- Treat language generation as an optional downstream rendering step.  
- Represent retrieval (RAG) as a tool invoked by the predictor, not as the cognitive core.  
- Allow multiple encoders/decoders to interoperate through a stable latent-state contract.

**Rationale**

- Local-first efficiency: always-on components can be smaller than full decoders.  
- Privacy boundaries: avoid unnecessary raw text duplication.  
- Tool-first execution: decisions precede narration.  
- Modular interfaces: swap encoders/decoders without changing the Control Plane.

**Consequence**

- Pros: lower always-on compute, cleaner tool routing, and better separation of state vs narration.  
- Cons/risks: latent state is harder to inspect; embedding collapse needs evaluation and guardrails.  
- This ADR defines an architectural stance, not a mandatory training objective.

---
