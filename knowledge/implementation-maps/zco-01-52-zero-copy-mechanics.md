---
id: ZCO-01
title: 52 Zero Copy Mechanics
type: implementation-map
status: draft
implementation_state: specified_only
proposition: "\uFEFF# 52 – Zero-Copy Mechanics DOC-ID: ZCO-01 Zero-copy operations over Arrow buffers (see 95-theory for proofs)."
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/52-zero-copy-mechanics.md
  heading: 52 Zero Copy Mechanics
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 52 Zero Copy Mechanics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

﻿# 52 – Zero-Copy Mechanics
DOC-ID: ZCO-01

Zero-copy operations over Arrow buffers (see `95-theory` for proofs). Outputs mutate booleans/indices; text moves exactly once into an LLM buffer.
