---
id: WORKFLOW-LEARNING-01
title: Execution trajectories and workflow-template learning
type: specification
status: draft
implementation_state: specified_only
proposition: Goni should represent completed delegated work as bounded execution trajectories and may infer reusable workflow-template candidates from repeated similar trajectories, while template learning updates procedural expectations rather than delegated authority.
domains:
- specs
- memory
- delegation
aliases:
- execution-trajectory
- workflow-template
relations:
- type: depends_on
  target: ANTICIPATORY-DELEGATION-01
- type: refines
  target: CDC-01
sources:
- SRC-FELDMAN2003-ORGANIZATIONAL-ROUTINES
- SRC-CERAVOLO2024-PREDICTIVE-PROCESS-MONITORING
artifacts: []
uncertainty: Similarity criteria, promotion thresholds, and template representation are research questions and must be calibrated against real or synthetic trajectory corpora.
legacy: []
---

# Execution trajectories and workflow-template learning

A completed unit of delegated work should yield a bounded trajectory record that
can be reconstructed from existing canonical objects and receipts:

[
\tau = (trigger, state, objective, actions, tools, approvals, corrections,
exceptions, outcome).
]

The trajectory is an observation of one concrete performance. Repeated similar
trajectories may support a reusable `WorkflowTemplate` candidate describing
the generalized routine, expected transitions, decision points, exception
classes, and relevant contextual predicates.

The distinction is deliberate:

- **trajectory**: what happened in one case;
- **workflow template**: a generalized procedural hypothesis about what usually
  happens across comparable cases.

Correction deltas remain especially strong learning signals and continue to be
handled by the Correction Delta Compiler. Acceptance, rejection, undo,
exception handling, and successful completion may also inform workflow
statistics.

Workflow-template promotion must be scoped, receipted, reviewable, and
reversible. Promotion may change procedural memory or prediction confidence; it
does not grant new tools, capabilities, mandates, corridors, budgets, or
external action authority.
