---
id: SKILL-EVAL-01
title: Selective Skill Materialization Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Compare loading all eligible skills, selecting whole skills, and selecting dependency-closed skill fragments to test whether selective materialization reduces instruction load while preserving task quality, verification behavior, and tool-selection correctness.
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
uncertainty: Skill routing can fail through false negatives even when average token savings are large. Evaluation must emphasize omitted decisive procedures and verification rules rather than average compression alone.
legacy: []
---

# Selective Skill Materialization Evaluation

## Baselines

For matched Work Orders and model/runtime conditions compare:

1. every eligible skill fully materialized;
2. top-K whole-skill materialization;
3. fragment-selective materialization with dependency closure.

## Metrics

Report:

- task quality;
- instruction-following accuracy;
- verification-step completion;
- tool-selection correctness;
- false-negative omission of decisive procedural rules;
- false-positive irrelevant-fragment load;
- materialized skill tokens;
- context-construction latency;
- end-to-end latency;
- user-visible error or rework rate.

## Promotion criterion

Fragment selection advances only when it reduces materialized instruction cost
without exceeding predefined false-negative, quality, verification, or
tool-selection thresholds on the named task class.
