---
id: SYS-02
title: Performance Realism for Sequence Architectures
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: SYS-02 Status: Specified only / roadmap This document defines how Goni evaluates claims about alternative sequence architectures (sub-quadratic attention, state-space, recurrent hybrids).'
domains:
- system
aliases: []
relations:
- type: synthesizes
  target: GONI-EXPERIMENT-5ADAC18F09F9
- type: synthesizes
  target: GONI-SYNTHESIS-7CD00D3DE959
- type: synthesizes
  target: GONI-SYNTHESIS-72436719F428
- type: synthesizes
  target: GONI-SYNTHESIS-8D3B01E32375
- type: synthesizes
  target: GONI-SYNTHESIS-A81243CF648D
- type: synthesizes
  target: GONI-SYNTHESIS-3C7B7CA4B3D1
- type: synthesizes
  target: GONI-SYNTHESIS-8C9FCA3EAC3F
- type: synthesizes
  target: GONI-SYNTHESIS-3E38C9AF52DA
- type: synthesizes
  target: GONI-SYNTHESIS-3A32F81B6EC0
- type: synthesizes
  target: GONI-SYNTHESIS-AB190C9135CA
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/30-performance.md
  heading: Performance Realism for Sequence Architectures
  revision: 01e3ecf4470f955ee157ca014244a88b47f6eb43
---

# Performance Realism for Sequence Architectures

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Performance Realism for Sequence Architectures
DOC-ID: SYS-02
Status: Specified only / roadmap

This document defines how Goni evaluates claims about alternative sequence
architectures (sub-quadratic attention, state-space, recurrent hybrids).

Goal: keep architecture decisions grounded in realistic tokenizer regimes,
scale, wall-clock performance, and product-level constraints.
