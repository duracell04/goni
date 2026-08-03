---
id: GONI-IMAP-8C1BEA9FE908
title: Dependency hygiene
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Minimise new dependencies.
domains:
- agent
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/AGENTS.md
  heading: Dependency hygiene
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Dependency hygiene

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Dependency hygiene
- Minimise new dependencies. If adding a dependency, document:
  - why it is needed
  - which crate owns it
  - what contract/invariant it supports
