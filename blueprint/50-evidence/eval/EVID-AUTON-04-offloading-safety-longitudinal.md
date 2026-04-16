---
id: EVID-AUTON-04
type: EVIDENCE
status: specified_only
---
# EVID-AUTON-04 Offloading Safety (Longitudinal)

Goal: track long-horizon outcomes of digital task offloading, including gains in
time recovery and risks such as over-delegation or deskilling.

## Core checks
- autonomous execution rate trend
- human oversight minutes trend
- override frequency and autonomy downgrades
- periodic audit findings on unnoticed policy drift
- delegation success rate from vague prompt to correct deliverable within a
  bounded number of turns
- downgrade frequency by failure mode (`lazy_agent`, `overcautious_agent`,
  `shape_shifter`, `complacency_engine`, `hidden_assumption_executor`)
- deskilling/offloading risk trends and recovery after policy changes

## Benchmark shape
- longitudinal replay on recurring workflows with policy-bundle promotion and
  rollback
- compare successive delegation-policy bundles on success, regret, overrides,
  and surfaced-assumption coverage
- report whether promotion improves throughput without increasing unnoticed
  policy drift

## Artifact links
- TBD
