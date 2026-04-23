---
id: EVID-LONGCTX-03
type: EVIDENCE
status: specified_only
---
# EVID-LONGCTX-03 Recursion-Budget Safety

Goal: verify that recursive or parallel corpus-reading stays inside explicit
budgets and does not become an unbounded orchestration path.

## Scenario
- Run corpus-reading and hybrid strategies under fixed latency, token, and tool
  budgets.
- Stress recursive decomposition on tasks that tempt over-scanning.
- Record whether the strategy degrades relative to the native in-window
  baseline on shorter inputs.

## Metrics
- recursion_depth
- subread_count
- tool-call count
- latency budget breach rate
- token budget breach rate
- over-scan rate
- underperforms-native-window rate

## Failure modes
- over-scans and wastes budget
- recursive decomposition drifts from task
- underperforms the base model inside the native context window

## Promotion gate
- Do not promote this lane into normative architecture until it shows
  repeatable wins on at least one Goni-relevant workload without unacceptable
  latency, audit, or safety regressions.

## Evidence artifact links
- TBD
