---
id: CTX-COMP-EVAL-01
title: Context Compilation Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Compare full-context, fixed top-K retrieval, existing submodular selection, and CTX-COMP-01 under matched model/runtime conditions to test whether a smaller compiled working set preserves task quality, decisive-evidence recall, citation fidelity, and authority-layer safety while reducing inference cost.
domains:
- research
- software
- system
aliases: []
relations:
- type: tests
  target: CTX-COMP-01
- type: tests
  target: GONI-PRINCIPLE-9062425CD490
sources:
- SRC-LIU2023-LOST-MIDDLE
- SRC-JIANG2023-LLMLINGUA
- SRC-JIANG2024-LONGLLMLINGUA
- SRC-LI2023-SELECTIVE-CONTEXT
artifacts: []
uncertainty: No superiority is assumed. The experiment must report where compilation loses to simpler baselines and must not promote thresholds across workloads or hardware without matched evidence.
legacy: []
---

# Context Compilation Evaluation

## Hypothesis

For at least some Goni workloads, a task-specific compiled working set can
reduce active tokens and runtime cost while preserving the information required
for correct, cited, authority-safe work.

## Baselines

Compare under the same model, runtime, hardware, decoding profile, and task set:

1. full available context within the native model window;
2. fixed top-K retrieval materialized directly;
3. existing submodular context selection under a fixed token budget;
4. CTX-COMP-01 with governed skill resolution, compression, omission tracking,
   and the same external evidence pool.

## Metrics

Report at minimum:

- task answer or decision quality;
- decisive-evidence Recall@k;
- citation/span fidelity;
- omission rate for decisive evidence;
- logical prompt tokens;
- fresh prefill tokens where measurable;
- TTFT and end-to-end latency;
- peak KV memory where measurable;
- compiler latency;
- compression distortion or reconstruction error;
- authorization violations attributable to compiled context: target zero.

## Promotion criterion

A compiler policy may be promoted only for a named workload and runtime profile
when it improves one or more resource metrics while remaining inside predefined
quality, evidence-recall, citation, and authority-safety thresholds.
