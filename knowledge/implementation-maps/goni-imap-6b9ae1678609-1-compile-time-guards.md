---
id: GONI-IMAP-6B9AE1678609
title: 1. Compile-Time Guards
type: implementation-map
status: draft
implementation_state: specified_only
proposition: define_tables! macro rejects LargeUtf8 in planes 𝒦/ℰ and enforces plane/table pairing. Clippy lint forbid(non_arrow_entity) to block non-Arrow domain structs crossing crate boundaries. Newtypes for IDs prevent accidental use of raw UUIDs across
domains:
- data
- software
- validation
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/80-validation-and-ci.md
  heading: 1. Compile-Time Guards
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 1. Compile-Time Guards

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Compile-Time Guards
- `define_tables!` macro rejects `LargeUtf8` in planes 𝒦/ℰ and enforces plane/table pairing.
- Clippy lint `forbid(non_arrow_entity)` to block non-Arrow domain structs crossing crate boundaries.
- Newtypes for IDs prevent accidental use of raw UUIDs across APIs.
