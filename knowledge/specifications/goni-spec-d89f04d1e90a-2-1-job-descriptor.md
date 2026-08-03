---
id: GONI-SPEC-D89F04D1E90A
title: 2.1 Job descriptor
type: specification
status: draft
implementation_state: specified_only
proposition: 'Required fields: job_id job_class (interactive | background | maintenance) slo_profile budget_set cancel_policy capability_set_id trace_id'
domains:
- agent
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md
  heading: 2.1 Job descriptor
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 2.1 Job descriptor

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Job descriptor

Required fields:
- `job_id`
- `job_class` (`interactive` | `background` | `maintenance`)
- `slo_profile`
- `budget_set`
- `cancel_policy`
- `capability_set_id`
- `trace_id`
