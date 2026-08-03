---
id: GONI-SPEC-F23F093D4A9A
title: 1. Purpose and scope
type: specification
status: draft
implementation_state: specified_only
proposition: 'SS-01 defines: the terminology used in the symbolic layer, the arbitration contract for proposal -> validation -> execution -> commit, the authority model for who can write which symbolic namespaces, and minimal failure semantics and constraint payload shape.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/symbolic-substrate.md
  heading: 1. Purpose and scope
  revision: 492528ae2a7ceb77ab6710043701423d31336c8f
---

# 1. Purpose and scope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Purpose and scope

SS-01 defines:
- the terminology used in the symbolic layer,
- the arbitration contract for proposal -> validation -> execution -> commit,
- the authority model for who can write which symbolic namespaces, and
- minimal failure semantics and constraint payload shape.

Non-goals:
- full logic programming, HTN planners, or a complete policy language.
- storage layout changes (F_sparse remains map<utf8, utf8>).
