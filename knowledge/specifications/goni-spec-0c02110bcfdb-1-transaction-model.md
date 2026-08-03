---
id: GONI-SPEC-0C02110BCFDB
title: 1. Transaction model
type: specification
status: draft
implementation_state: specified_only
proposition: 'Each mutating tool operation executes in a transaction lifecycle: prepared -> executing -> committed | rolled_back | compensating | failed.'
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md
  heading: 1. Transaction model
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 1. Transaction model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Transaction model

Each mutating tool operation executes in a transaction lifecycle:
`prepared -> executing -> committed | rolled_back | compensating | failed`.

`failed` indicates operator attention is required and compensation did not fully
restore intended postconditions.
