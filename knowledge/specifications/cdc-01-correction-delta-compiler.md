---
id: CDC-01
title: Correction Delta Compiler
type: specification
status: draft
implementation_state: specified_only
proposition: The Correction Delta Compiler converts differences between agent outputs and principal-approved outcomes into scoped, receipted, reviewable learning updates; together with execution-trajectory evidence it may refine procedural expectations and workflow templates while preserving the rule that learning never creates delegated authority.
domains:
- specs
aliases:
- CORRECTION-DELTA-COMPILER
relations:
- type: depends_on
  target: ANTICIPATORY-DELEGATION-01
sources:
- SRC-FELDMAN2003-ORGANIZATIONAL-ROUTINES
artifacts: []
uncertainty: The original correction-learning design is retained; trajectory clustering and workflow-template promotion remain specified only and require evaluation.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: Correction Delta Compiler
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# Correction Delta Compiler

> Status boundary: this is a draft specification. Present-tense or enforcement
> language states intended contract behavior, not observed implementation,
> verification, or non-bypassability.

The Correction Delta Compiler (CDC) converts differences between agent outputs
and principal-approved outputs or outcomes into scoped, receipted, reviewable
updates to procedural memory, delegation policy proposals, skills, harness
rules, workflow templates, and regression tests.

Goni should learn from deltas: the difference between what the agent produced
and what the principal corrected, accepted, rejected, sent, repeated, undid, or
complained about.

`WORKFLOW-LEARNING-01` adds completed execution trajectories as a second
procedural-learning evidence lane. Similar trajectories may support a
WorkflowTemplate candidate, while correction deltas remain especially strong
evidence about where a generalized routine should change.

Learning and authority remain separate. A learned preference, repeated
acceptance pattern, or high-confidence WorkflowTemplate may alter prediction or
preparation behavior within an existing corridor. It MUST NOT silently grant a
new capability, widen a mandate, remove an approval requirement, or otherwise
create authority.
