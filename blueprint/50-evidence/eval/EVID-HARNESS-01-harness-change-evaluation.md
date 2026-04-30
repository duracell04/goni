---
id: EVID-HARNESS-01
type: EVID
status: specified_only
---
# Harness Change Evaluation

This evidence lane evaluates changes to the Goni Harness Plane: prompts,
context assembly templates, retrieval policies, routing rules, tool manifests,
approval corridors, receipt formats, and evaluation packs.

Harness changes are not accepted because they feel better. Each change must be
represented as an explicit artefact with a predicted effect, receipt-backed
evidence, an evaluation window, retention criteria, and a rollback condition.

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

## Metrics

Minimum reported metrics:

- task success delta,
- approval rate delta,
- user edit distance delta,
- override and rejection rates,
- rollback rate and rollback success rate,
- token cost and latency delta,
- user interruption rate,
- failed retrieval rate,
- wrong-routing rate,
- policy violation rate,
- transfer across model families or model routes when applicable.

## Evidence sources

Receipts are the source evidence. Experience digests and harness-change
manifests are derived artefacts that summarize receipts, edits, approvals,
overrides, rejections, failed retrievals, wrong-routing choices, cost, latency,
policy decisions, and rollback events.

Raw logs may be retained for drill-down, but promotion decisions should use
bounded, structured evidence so harness evolution does not become unstructured
trajectory replay.

## Acceptance criteria

- The target component and seam are named.
- The expected outcome deltas are stated before evaluation.
- The evidence window is fixed before evaluation.
- Safety, policy, egress, latency, and cost regressions are reported.
- Retention or rollback is decided from the declared criteria.
- The result links back to receipts or reproducible replay artefacts.

## Related docs

- [Delegation Doctrine](/blueprint/10-product/15-delegation-doctrine.md)
- [Learning Loop](/blueprint/20-system/50-learning-loop.md)
- [Software Architecture](/blueprint/software/20-architecture.md)
- [Metrics scorecard](/blueprint/docs/metrics.md)
