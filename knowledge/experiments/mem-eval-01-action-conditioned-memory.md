---
id: "MEM-EVAL-01"
title: "Action-conditioned persistent memory evaluation"
type: "experiment"
status: "draft"
implementation_state: "not_applicable"
proposition: "Evaluate whether GONI-governed persistent memory improves multi-session retrieval and authorized downstream task performance against no-memory and simpler-memory baselines while preserving temporal correctness, provenance, authorization, deletion behavior and total-system resource budgets."
domains:
- "agent"
- "memory"
- "research"
- "system"
aliases: []
relations:
- type: "tests"
  target: "MEM-RETR-01"
- type: "tests"
  target: "CGG-01"
- type: "tests"
  target: "SKILL-REG-01"
- type: "derived_from"
  target: "EVID-MEM-ACTION2026"
sources:
- "SRC-WU2025-LONGMEMEVAL"
- "SRC-HE2026-MEMORYARENA"
- "SRC-SHEN2026-MEM2ACTBENCH"
- "SRC-LIN2026-LTMSEC"
- "SRC-LATIMER2026-HINDSIGHT"
- "SRC-ZHANG2026-HIVEMEM"
artifacts: []
uncertainty: "The proposed evaluation has not run. Its applicability to GONI's hardware, workload distribution, memory privileges, and user goals is unmeasured; success criteria must be declared before benchmark execution."
legacy: []
---

# MEM-EVAL-01 — Action-conditioned persistent memory evaluation

## Question and hypothesis

Does canonical GONI memory improve delayed multi-session decisions and task completion beyond no-memory and simpler retrieval baselines, at acceptable total system cost and without unauthorized memory writes, leakage, hallucinated provenance, or consequential side effects?

## Controlled conditions

For identical held-out tasks, tools, authorization profiles, model/runtime, and source observations, compare:

1. **No durable memory:** session-only context.
2. **Transcript search:** bounded keyword/vector retrieval against past observations.
3. **Governed structured memory:** `MEM-RETR-01` plus typed factual/derived/hypothesis records and canonical source references.
4. **Temporal context:** condition 3 plus `CGG-01` graph/temporal retrieval over source-grounded claims.
5. **Selective procedures and multimodal evidence:** condition 4 plus eligible `SKILL-REG-01` procedures and source-linked multimodal evidence when task-relevant.

Use a mixed fixture set: adapted LongMemEval-style recall, MemoryArena-style interdependent sequences, Mem2ActBench-style tool grounding, and realistic authorized GONI Work Orders. Benchmark-specific metrics stay separated; direct score comparisons require a matched measurement protocol.

## Scenario coverage

- factual extraction and cross-session synthesis;
- updates, superseded facts, conflicting testimony, recording time versus validity time;
- abstention, unknowns, synthetic-hypothesis contamination, source-lineage duplication;
- handoff between different models or agent runtimes while preserving pending steps;
- verified correction-to-procedure compilation with explicit review;
- query-dependent text, image and audio evidence selection;
- memory poisoning attempts, privacy-scope crossover, grant revocation, forgetting and reindexing;
- concurrent writes, version conflicts, interruption/replay and stale cache reads;
- tasks where available memory is irrelevant or misleading.

## Measures

Report retrieval precision/recall, source attribution, temporal validity, contradiction/uncertainty calibration, justified abstention, task completion, tool-selection and parameter-grounding accuracy, false-positive memory application, time to useful answer, and retained-handoff fidelity. Separately measure approval requests, unauthorized effects, cross-principal leakage, memory write violations, deleted-content resurfacing, and rollback/recovery success.

Track memory index build/update cost, query latency, tokens, compute and energy where instrumented, bytes stored/transferred, background interference, human verification time, and total cost per **verified completed task**. State measurement coverage, confidence intervals or variability, and all observed failures.

## Acceptance protocol

Predeclare a representative task distribution, critical failure thresholds, resource ceilings, and statistically meaningful improvement requirements before collecting results. Prefer the simplest pipeline that meets held-out performance and policy objectives. Require zero observed unauthorized effects and privacy-scope leaks in the test suite, while recognizing that finite tests cannot prove absence of future failures.

Memory updates require authorized write gates. Canonical sources and receipts remain reconstructable under applicable retention policy; proposed deletion must be verified against canonical storage **and all derived indexes/caches**. Do not infer that provenance standards alone enforce this.

## Execution boundary

Executable scenarios and benchmark artifacts belong in [goni-prototype-lab](https://github.com/duracell04/goni-prototype-lab). This blueprint experiment creates no runtime dependency, model authority, new canonical database, or relaxation of GONI's kernel mediation. Negative and neutral results are first-class evidence.
