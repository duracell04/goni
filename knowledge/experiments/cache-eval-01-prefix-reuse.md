---
id: CACHE-EVAL-01
title: Prefix and KV Reuse Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Measure prefix and KV reuse under exact compatible and deliberately incompatible inference frames to verify latency gains, cache accounting, and invalidation correctness without semantic or authority leakage.
domains:
- research
- software
- system
aliases: []
relations:
- type: tests
  target: CACHE-01
sources:
- SRC-GIM2024-PROMPT-CACHE
- SRC-ZHENG2024-SGLANG
- SRC-KWON2023-VLLM
artifacts: []
uncertainty: Cache behavior is backend-specific. The experiment must treat incorrect reuse and missed invalidation as correctness failures even when output appears plausible.
legacy: []
---

# Prefix and KV Reuse Evaluation

## Test matrix

Construct repeated calls that independently vary:

- model bundle;
- adapter or learned-prefix stack;
- tokenizer;
- chat template;
- stable system/policy prefix;
- tool schema;
- exact reusable prefix token sequence;
- runtime KV layout or backend profile.

Include positive cases where reuse is valid and negative cases where one
compatibility dimension changes.

## Metrics

Report:

- eligible-cache hit rate;
- false cache hit rate;
- false invalidation rate;
- invalidation reason accuracy;
- logical prompt tokens;
- fresh prefill tokens;
- reused prefix tokens;
- TTFT;
- end-to-end latency;
- peak KV memory;
- output equivalence under deterministic profiles where meaningful;
- authority-policy violations.

A false cache hit is a correctness defect independent of latency savings.

## Promotion criterion

A cache policy may be promoted for a runtime profile only when compatibility
and invalidation tests pass and measured reuse yields repeatable benefit.
