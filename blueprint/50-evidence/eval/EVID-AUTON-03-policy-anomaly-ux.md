---
id: EVID-AUTON-03
type: EVIDENCE
status: specified_only
---
# EVID-AUTON-03 Policy-and-Anomaly UX Evaluation

Goal: validate that users can govern autonomy primarily through policies and
exception handling, not per-action confirmation loops.

## Core checks
- human oversight minutes per day
- anomaly review completion time
- policy edit success rate
- reduction in per-action approval interactions
- initiative calibration: rate of user "stop doing that" corrections or forced
  downgrades
- anomaly-handling UX for surfaced assumptions, uncertainty, and tool intent
- user override burden for over-questioning or under-explained autonomy

## Benchmark shape
- replay operator sessions with policy edits, anomalies, and clarification
  events
- measure which interruptions were decisive versus unnecessary
- compare review throughput when receipts include delegation metadata versus
  minimal metadata

## Artifact links
- TBD
