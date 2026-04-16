---
id: EVID-AUTON-02
type: EVIDENCE
status: specified_only
---
# EVID-AUTON-02 Risk Threshold Calibration

Goal: calibrate risk scoring and threshold policies so "auto unless risky"
improves throughput without unsafe autonomous side effects.

## Core checks
- risk score calibration on labelled traces
- escalation precision/recall for high-risk actions
- unsafe autonomy incident rate under production-like policies
- rollback/compensation success for autonomous mistakes
- assumption visibility rate for actions taken under ambiguity
- uncertainty calibration: high-uncertainty episodes should correlate with
  escalation, review, or explicit assumption surfacing
- override-triggered regret: measure quality minus turns, latency, and user
  corrections after user override

## Benchmark shape
- replay the same trace set with different clarification and threshold bundles
- compare `risk_score`, `uncertainty_level`, and final `delegation_outcome`
- inspect whether hidden assumptions predict later overrides or unsafe commits

## Artifact links
- TBD
