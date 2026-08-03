---
id: EVID-HARNESS-02
type: EVID
status: specified_only
---
# Correction Delta Compiler Evaluation

This evidence lane evaluates whether the Correction Delta Compiler converts
principal corrections into scoped, receipted, reviewable procedural memory and
harness rules without overfitting or hidden personalization drift.

## Goal

Prove through trace replay that correction deltas improve future behavior only
through governed seams: memory, retrieval, prompt assembly, harness policy, or
approved promotion datasets. The base model is not assumed to learn online.

## Required replay scenarios

- A single correction creates a scoped hypothesis, not a global preference.
- Repeated consistent corrections increase confidence only within matching
  scope.
- Contradictory corrections reduce confidence, narrow scope, or require review.
- Accepted learning emits `MemoryEntry + Receipt + RegressionTest`.
- Learning receipts omit raw user, draft, and correction text by default.
- High-risk, privacy, legal, financial, or constitutional preferences require
  explicit approval before promotion.
- Retrieval, prompt, routing, or policy behavior changes attach only to declared
  Learning Loop seams.
- Replayed tasks show lower correction distance without higher rejection,
  override, interruption, policy violation, or privacy-risk rates.

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

## Metrics

Minimum reported metrics:

- user edit distance before and after accepted learning,
- acceptance, rejection, override, and complaint rates,
- false global-preference promotion rate,
- contradiction detection rate,
- stale or expired preference retrieval rate,
- raw-content leakage rate in receipts,
- learning-card approval and rejection rates,
- regression pass rate for accepted learned rules,
- policy violation, privacy-risk, and interruption deltas,
- rollback success rate for rejected or regressed learned rules.

## Acceptance criteria

- A correction-derived update names its scope, evidence, contradiction count,
  review status, and target seam.
- A single correction cannot become a global stable default.
- Accepted updates link to memory diff refs and learning receipts.
- Learning receipts include source refs and summaries, not raw text by default.
- High-risk and constitutional updates require explicit approval.
- Regression tests fail if the system reintroduces the corrected behavior in
  matching scope.
- Replay shows improvement without safety, privacy, latency, or interruption
  regression.

## Related docs

- [Correction Delta Compiler](/blueprint/30-specs/correction-delta-compiler.md)
- [Learning Loop](/blueprint/20-system/50-learning-loop.md)
- [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Harness Change Evaluation](/blueprint/50-evidence/eval/EVID-HARNESS-01-harness-change-evaluation.md)
