---
id: GONI-PRINCIPLE-B40DDEFD1872
title: '0.1 Empirical motivation: context is not memory'
type: principle
status: draft
implementation_state: specified_only
proposition: 'Long-context evaluations show positional sensitivity: evidence placed in the middle of a long prompt is used less reliably than evidence near boundaries, and accuracy drops as prompts grow and become more diffuse.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/10-axioms-and-planes.md
  heading: '0.1 Empirical motivation: context is not memory'
  revision: 43a497b2a7deb59e07ad598a7c0496fbc9dc3cbe
---

# 0.1 Empirical motivation: context is not memory

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 0.1 Empirical motivation: context is not memory

Long-context evaluations show positional sensitivity: evidence placed in the
middle of a long prompt is used less reliably than evidence near boundaries,
and accuracy drops as prompts grow and become more diffuse. This implies the
context window behaves more like scarce working memory than durable storage.
Systems that accumulate transcripts in-context risk signal dilution and
re-introducing stale or speculative text. [[liu2023-lost-middle]]

This motivates the following invariants: the TXT axiom (raw text confinement)
and plane separation, so durable state lives in the Arrow spine while the
Context plane is a bounded, curated projection.
