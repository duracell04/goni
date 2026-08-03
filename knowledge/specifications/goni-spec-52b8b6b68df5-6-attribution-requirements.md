---
id: GONI-SPEC-52B8B6B68DF5
title: 6. Attribution requirements
type: specification
status: draft
implementation_state: specified_only
proposition: 'All egress and mutating writes must be attributable to: trace_id span_id capability_id policy_hash job_id Attribution metadata is mandatory in receipts.'
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-ENF-01-non-bypassable-mediation.md
  heading: 6. Attribution requirements
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 6. Attribution requirements

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Attribution requirements

All egress and mutating writes must be attributable to:
- `trace_id`
- `span_id`
- `capability_id`
- `policy_hash`
- `job_id`

Attribution metadata is mandatory in receipts.
