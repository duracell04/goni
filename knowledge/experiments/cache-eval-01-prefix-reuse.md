---
id: CACHE-EVAL-01
title: Prefix and KV Reuse Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Evaluate CACHE-01 compatibility and reuse rules by measuring exact cache eligibility, invalidation correctness, fresh prefill reduction, TTFT, KV memory, and output equivalence under deterministic profiles where equivalence is meaningful.
domains:
- research
- software
- system
aliases: []
relations:
- type: tests
  target: CACHE-01
- type: tests
  target: INF-FRAME-01
sources:
- SRC-GIM2024-PROMPT-CACHE
- SRC-ZHENG2024-SGLANG
- SRC-KWON2023-VLLM
artifacts: []
uncertainty: Cache behavior is backend-specific. A passing policy for one model, tokenizer, runtime, or KV layout does not establish compatibility for another.
legacy: []
---

# Prefix and KV Reuse Evaluation

## Cases

Exercise exact reuse and deliberate invalidation across changes to:

- model bundle;
- adapter or learned-prefix stack;
- tokenizer;
- chat template;
- system/policy prefix;
- tool-schema prefix;
- exact prefix token sequence;
- position semantics;
- runtime KV representation.

## Metrics

Report:

- cache eligibility precision and recall;
- cache-hit rate;
- invalidation correctness;
- logical prompt tokens;
- fresh prefill tokens;
- reused prefix tokens;
- TTFT;
- decode latency;
- peak KV memory;
- deterministic output equivalence where the runtime supports a meaningful
  matched comparison;
- authority violations caused by reuse: target zero.

Semantic similarity alone must never count as a successful compatibility test.
