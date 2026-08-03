---
id: GONI-SPEC-2E3DADF1A57B
title: 1. Purpose
type: specification
status: draft
implementation_state: specified_only
proposition: '"Two network personalities" is an information-flow control problem, not a UI toggle.'
domains:
- network
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/network-gate-and-anonymity.md
  heading: 1. Purpose
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 1. Purpose

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Purpose

- "Two network personalities" is an information-flow control problem, not a UI
  toggle. The system must enforce where bytes may go under a declared policy.
- Networking is a governed side effect, analogous to Tool Capability API (TOOL-01).
