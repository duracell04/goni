---
id: CDC-01
title: Correction Delta Compiler
type: specification
status: draft
implementation_state: specified_only
proposition: 'DOC-ID: CDC-01 Status: Specified only / roadmap The Correction Delta Compiler (CDC) converts differences between agent outputs and principal-approved outputs into scoped, receipted, reviewable updates to procedural memory, delegation policy, skills, harness rules, and regression tests.'
domains:
- specs
aliases:
- CORRECTION-DELTA-COMPILER
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: Correction Delta Compiler
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# Correction Delta Compiler

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Correction Delta Compiler
DOC-ID: CDC-01

Status: Specified only / roadmap

The Correction Delta Compiler (CDC) converts differences between agent outputs
and principal-approved outputs into scoped, receipted, reviewable updates to
procedural memory, delegation policy, skills, harness rules, and regression
tests.

CDC is not ordinary memory storage. It is an online preference-estimation and
policy-updating subsystem. Goni should remember facts, but it should learn taste
from deltas: the difference between what the agent produced and what the
principal corrected, accepted, rejected, sent, repeated, or complained about.
