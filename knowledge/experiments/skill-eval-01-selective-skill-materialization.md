---
id: SKILL-EVAL-01
title: Selective Skill Materialization Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Compare loading all available procedural skills, routing complete top-K skills, and selectively materializing skill fragments to measure whether fragment-level routing reduces instruction cost without losing decisive procedure or verification requirements.
domains:
- research
- agent
- software
aliases: []
relations:
- type: tests
  target: SKILL-REG-01
sources: []
artifacts: []
uncertainty: Fragment-level routing may introduce false-negative procedural omissions. The experiment must measure those failures directly rather than assuming smaller instruction sets are superior.
legacy: []
---

# Selective Skill Materialization Evaluation

## Conditions

Compare:

1. all eligible skills fully materialized;
2. routed top-K skills fully materialized;
3. routed skill fragments with dependency closure under SKILL-REG-01.

Use matched Work Orders, model stack, tools, authority, evidence, and output
contracts.

## Metrics

Report:

- skill and fragment tokens materialized;
- routing precision and recall;
- decisive-procedure omission rate;
- verification-contract coverage;
- tool-selection accuracy;
- task quality;
- instruction-conflict rate;
- latency and fresh prefill cost;
- unnecessary skill load;
- authority-policy violations.

The test corpus SHOULD include tasks where a rarely used verification fragment
is decisive, so frequency alone cannot appear successful by dropping low-rate
but high-consequence instructions.

## Promotion criterion

Fragment-selective materialization advances only when token or latency savings
remain inside task-quality, procedural-recall, and authority-safety thresholds
for a named workload.
