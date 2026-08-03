---
id: GONI-EXPERIMENT-1F8811AD87C7
title: Required change record
type: experiment
status: draft
implementation_state: not_applicable
proposition: Required change record
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-HARNESS-01-harness-change-evaluation.md
  heading: Required change record
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# Required change record

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Required change record

```yaml
harness_change:
  id: "short-stable-id"
  target_component: "retrieval_policy | routing_policy | approval_corridor | ..."
  target_seam: "S1 | S3 | S4 | S5"
  prediction:
    task_success_delta: "+N%"
    approval_rate_delta: "+N%"
    user_edit_distance_delta: "-N%"
    negative_feedback_delta: "no increase"
  evidence_refs:
    - receipt_id: "rec_..."
    - eval_run_id: "eval_..."
  eval_window: "next N matching tasks or replay suite version"
  retention_criteria: "prediction holds without safety or latency regression"
  rollback_condition: "metric crosses declared threshold"
```
