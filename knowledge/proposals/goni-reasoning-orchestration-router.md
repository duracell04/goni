---
id: GONI-PROPOSAL-REASON-ROUTER
title: Evidence-calibrated reasoning orchestration over atomic theory nodes
type: proposal
status: draft
implementation_state: specified_only
proposition: Explore retrieval and composition of atomic reasoning nodes through the Goni knowledge graph, with conditional invocation of prompts, solvers and specialized models based on task value, cost, and measured reliability.
domains:
  - reasoning
  - agent
  - system
aliases: []
relations:
  - type: depends_on
    target: GONI-PRINCIPLE-REASON-COMPOSE
  - type: refines
    target: GONI-DECISION-9AF49466170D
sources: []
artifacts: []
uncertainty: Weighted graph ranking, solver contracts, routing accuracy, cost effectiveness, and model selection require experiments.
legacy: []
---

# Reasoning orchestration research proposal

## Atomic data contract

Represent each reusable theory as an addressable node with identity, proposition, diagnostic signals, formal assumptions, inputs, candidate methods, expected outputs, limitations, provenance, and links. Treat weights as context-dependent retrieval scores or evidence strengths with explicit semantics and provenance, rather than intrinsic truth values.

## Proposed flow

1. Parse decision question, constraints, evidence, and desired output.
2. Retrieve candidate reasoning nodes from typed graph edges and semantic relevance.
3. Evaluate assumptions and possible complementary or contradictory lenses.
4. Route to the smallest sufficient combination of conceptual skill, deterministic solver, or specialized model; use measured task-specific performance and resource budgets.
5. Synthesize options and uncertainty; verify exact computations independently.
6. Pass proposed consequential actions to the kernel's canonical policy/capability authority; emit receipts for authorized effects.
7. Record outcome evidence and revise retrieval/routing hypotheses under reviewable provenance.

## Governance boundary

Graph weights, embeddings, model judgments, and learned routers may guide cognition; canonical permissions, revocations, and execution authority remain deterministic and kernel-owned. Keep learned evidence scores separate from confidence calibration, causal identification and legal authority.

## Success criteria

Compare against a single-model baseline and fixed-theory baseline on matched tasks: correct problem decomposition, assumption coverage, decision utility, constraint violations, solver correctness, inference cost, latency, and unnecessary theory invocation. Track errors due to overcomposition as well as missed relevant lenses.
