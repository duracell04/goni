---
id: GONI-EXPERIMENT-23092C379EE5
title: Required artifacts
type: experiment
status: draft
implementation_state: not_applicable
proposition: Required artifacts
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-HARNESS-02-correction-delta-compiler.md
  heading: Required artifacts
  revision: f91b4339e701c60c0d3508cf2bb3bd613ef2e36d
---

# Required artifacts

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Required artifacts

```yaml
correction_delta_case:
  case_id: "cdc_case_..."
  task_class: "..."
  draft_ref: "prompt_or_output_hash"
  final_ref: "approved_output_hash"
  delta_classification:
    - "tone_correction"
    - "structure_correction"
  proposed_rule_ref: "rule_..."
  scope: "global | project | channel | recipient | task_class | session"
  confidence: 0.0
  evidence_count: 1
  contradiction_count: 0
  review_status: "pending | accepted | rejected | limited"
  receipt_refs:
    - "rec_..."
  memory_diff_refs:
    - "memdiff_..."
  regression_test_refs:
    - "replay_..."
```
