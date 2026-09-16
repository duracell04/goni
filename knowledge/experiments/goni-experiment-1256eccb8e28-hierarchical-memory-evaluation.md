---
id: GONI-EXPERIMENT-1256ECCB8E28
title: Hierarchical Memory Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Evaluate whether recoverable hierarchical memory preserves task-relevant state and correct-next-action quality better than repeated lossy compaction at comparable context and latency budgets.
domains:
- evaluation
- memory
aliases: []
relations:
- type: tests
  target: GONI-THESIS-AEA4F8746318
- type: tests
  target: GONI-SPECIFICATION-BB656A72DF7D
- type: tests
  target: GONI-SPECIFICATION-C64D8275BA34
sources:
- SRC-PACKER2023-MEMGPT
- SRC-KWON2023-PAGEDATTENTION
artifacts: []
uncertainty: This node specifies an evaluation protocol, not completed evidence. Thresholds and workload distributions must be fixed before execution.
legacy: []
---

# Hierarchical Memory Evaluation

The experiment compares three memory strategies over long-horizon delegated tasks:

1. a full-context oracle where feasible;
2. repeated lossy compaction without exact rehydration;
3. hierarchical recoverable memory with pinned state, exact archival state, retrieval, and semantic-page-fault handling.

The primary outcome is not token survival. It is downstream decision quality. A system-level target is:

`P(correct next action | active context + recoverable memory + authoritative state)`

A weighted critical-state recall metric may be used for facts or constraints with heterogeneous importance:

`R_w = Σ_i w_i · 1[f_i recoverable] / Σ_i w_i`

The evaluation should also measure task success, source and provenance recovery, semantic drift, contradictions detected before action, page-fault precision and recall, page-in latency, retrieval cost, unnecessary context tokens, and recovery of exact identifiers, files, commits, or prior decisions. Authority violations must be measured separately so memory quality cannot conceal privilege escalation.

A useful optimization framing is:

`J = E[D] + λE[L] + μC_compute + νC_storage`

where `D` is task distortion relative to the oracle, `L` is latency, and the remaining terms capture compute and storage costs. The coefficients must be declared before comparing systems.

The experiment should use fixed task traces and a predefined critical-state ledger so evaluation does not reward a system for retaining easy facts while losing high-consequence constraints. Repeated compaction should be evaluated across multiple compaction cycles to quantify cumulative semantic drift rather than a single summary operation.

Results become evidence only after the workload, implementation, seeds where applicable, hardware, model, retrieval configuration, budgets, and raw outputs are pinned as reproducible artifacts.
