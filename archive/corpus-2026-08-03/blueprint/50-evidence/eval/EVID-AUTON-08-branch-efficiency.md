---
id: EVID-AUTON-08
type: EVIDENCE
status: specified_only
---
# EVID-AUTON-08 Branch Efficiency Under Fixed Budgets

Goal: measure whether attention-density defaults preserve quality under fixed
token and latency budgets.

## Scenario
- Compare single-target prompts against multi-variant prompts on the same task.
- Fix input budget, latency budget, and retrieval set.
- Measure outcomes with and without branch minimization.

## Metrics
- branch_count
- variant_count_requested
- history_tokens
- context_tokens
- tool_schema_tokens
- output_tokens
- quality delta at equal budget

## Boundary
- This artifact stays focused on attention density and branching economics.
- Long-context reading strategy comparisons live in the `EVID-LONGCTX-*` lane
  and may reference this artifact for shared branch/variant metrics.

## Evidence artifact links
- TBD
