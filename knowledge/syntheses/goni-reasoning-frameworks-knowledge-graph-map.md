---
id: GONI-SYNTHESIS-REASON-MAP
title: Cross-disciplinary theory map for the Goni reasoning graph
type: synthesis
status: draft
implementation_state: not_applicable
proposition: The nine theory mappings form a composable, evidence-sensitive reasoning vocabulary connected to an optional orchestration proposal rather than nine isolated application-domain subsystems.
domains:
  - reasoning
  - system
aliases: []
relations:
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-GAME
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-AGENCY
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-ATTENTION
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-CAUSAL
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-CONTROL
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-OPT
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-NETWORK
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-MECHANISM
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-DYNAMICS
  - type: synthesizes
    target: GONI-PRINCIPLE-REASON-COMPOSE
  - type: synthesizes
    target: GONI-PROPOSAL-REASON-ROUTER
  - type: synthesizes
    target: GONI-EXPERIMENT-REASON-ROUTING
sources: []
artifacts: []
uncertainty: Diagnostic mappings are useful heuristics, not one-to-one classifications or evidence of working orchestration.
legacy: []
---

# Cross-disciplinary reasoning map

| Diagnostic | Candidate lens | Atomic node |
| --- | --- | --- |
| Conflicting interests | Game theory | GONI-PRINCIPLE-REASON-GAME |
| Misaligned incentives | Principal–agent theory | GONI-PRINCIPLE-REASON-AGENCY |
| Excessive inference and information-processing cost | Rational inattention | GONI-PRINCIPLE-REASON-ATTENTION |
| Unclear root cause | Causal inference | GONI-PRINCIPLE-REASON-CAUSAL |
| Unstable system | Control theory | GONI-PRINCIPLE-REASON-CONTROL |
| Inefficient resource allocation | Optimization theory | GONI-PRINCIPLE-REASON-OPT |
| Poor coordination | Network theory | GONI-PRINCIPLE-REASON-NETWORK |
| Dysfunctional institutional rules | Mechanism design | GONI-PRINCIPLE-REASON-MECHANISM |
| Changing environment | Dynamical systems and learning theory | GONI-PRINCIPLE-REASON-DYNAMICS |

These are entry points, not exclusive assignments. A single situation can require several lenses; theories may share variables and impose incompatible assumptions. The graph should preserve atomic propositions, typed links, uncertainty and provenance. A reasoning router is a proposed consumer of this graph, not the source of truth. See the compositional principle, orchestration proposal and evaluation experiment for distinct normative, implementation and testing claims.
