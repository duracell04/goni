---
id: CTX-COMP-EVAL-01
title: Context Compilation Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Compare authority-aware context compilation against full-context, fixed top-K retrieval, and existing submodular-selection baselines under matched model, runtime, task, and authority conditions.
domains:
- research
- software
- system
aliases: []
relations:
- type: tests
  target: CTX-COMP-01
sources:
- SRC-LIU2023-LOST-MIDDLE
- SRC-JIANG2023-LLMLINGUA
- SRC-JIANG2024-LONGLLMLINGUA
- SRC-LI2023-SELECTIVE-CONTEXT
artifacts: []
uncertainty: No performance advantage is assumed. The experiment must report workloads where compiler overhead, compression, or selection harms quality as well as workloads where it improves efficiency.
legacy: []
---

# Context Compilation Evaluation

## Hypothesis

For some Goni workloads, a compiled working set can reduce active-token,
latency, memory, or energy cost while preserving predefined task-quality,
decisive-evidence, citation, and authority-safety thresholds.

## Baselines

Compare under the same model bundle, runtime profile, task corpus, and available
source material:

1. full available context inside the native window;
2. fixed top-K governed retrieval;
3. existing reranking plus submodular context selection;
4. CTX-COMP-01 with governed skill, compression, and omission accounting.

Long-context tasks that exceed the native window SHOULD include a declared
reading or decomposition baseline rather than silently truncating the full
context condition.

## Metrics

Report at minimum:

- task quality under a named task-specific metric;
- decisive-evidence recall;
- citation/span fidelity;
- unsupported-claim rate;
- context tokens;
- compilation latency;
- time to first token;
- end-to-end latency;
- peak KV memory where observable;
- fresh prefill tokens where observable;
- energy or thermal proxy where practical;
- compression distortion or fidelity failures;
- omission reasons for decisive candidates;
- authority-policy violations.

Authority-policy violations have a target of zero. A lower token count does not
compensate for missing decisive evidence or unauthorized effects.

## Promotion criterion

A compiler policy may be promoted only for named workload and hardware profiles
where it demonstrates repeatable resource improvement inside predefined quality,
evidence, and safety thresholds. Negative or workload-specific results remain
valid evidence.
