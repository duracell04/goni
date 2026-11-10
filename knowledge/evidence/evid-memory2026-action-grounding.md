---
id: "EVID-MEM-ACTION2026"
title: "Memory recall and memory-grounded action are distinct evaluation targets"
type: "evidence"
status: "draft"
implementation_state: "not_applicable"
proposition: "LongMemEval measures retrieval and temporal/update abilities, whereas MemoryArena and Mem2ActBench evaluate memory use in interdependent tasks and tool parameter grounding; evaluating a persistent-agent memory system requires both retrieval and action-oriented outcomes."
domains:
- "agent"
- "memory"
- "research"
aliases: []
relations:
- type: "supports"
  target: "MEM-RETR-01"
- type: "supports"
  target: "GONI-SPEC-33294E0D3306"
sources:
- "SRC-WU2025-LONGMEMEVAL"
- "SRC-HE2026-MEMORYARENA"
- "SRC-SHEN2026-MEM2ACTBENCH"
artifacts: []
uncertainty: "The benchmarks test different tasks, datasets, and models, so their scores are not directly comparable. Source-reported findings do not imply that GONI passes any benchmark or has a working memory layer."
legacy: []
---

# Memory recall versus action-conditioned memory use

## Published benchmark observations

[LongMemEval (ICLR 2025)](https://proceedings.iclr.cc/paper_files/paper/2025/hash/d813d324dbf0598bbdc9c8e79740ed01-Abstract-Conference.html) tests information extraction, multi-session reasoning, temporal reasoning, knowledge updates and abstention across extended conversations.

[MemoryArena (ICML 2026)](https://proceedings.mlr.press/v306/he26am.html) evaluates interdependent multi-session agent tasks, in which knowledge from previous experience must inform later actions. The paper reports that strong results on recall-oriented benchmarks need not imply effective memory-driven task performance.

[Mem2ActBench (ACL 2026)](https://aclanthology.org/2026.acl-long.370/) tests whether agents can apply prior context to select tools and ground their parameters in persistent task-oriented interactions.

## GONI interpretation and boundary

Measure whether stored information is *correctly retrieved*, *correctly understood in temporal context*, and *correctly applied to a subsequent task*. Keep the consequences of execution separately subject to GONI's kernel capabilities and policy. A model's ability to propose an appropriate tool call does not give it authority to perform that call.

These papers supply complementary **evaluation designs**, not a combined leaderboard or evidence of GONI implementation. `MEM-EVAL-01` defines a matched GONI-specific test with additional governance, provenance, resource and negative-case measurements.
