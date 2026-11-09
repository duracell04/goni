---
id: "EXP-ADAPT-01"
title: "Local model specialization: quality, lifecycle cost, and hardware fit"
type: "experiment"
status: "draft"
implementation_state: "not_applicable"
proposition: "Evaluate whether task-specific local adaptation with Soup or another training backend reduces full-lifecycle cost at predeclared GONI task-quality, authority, and hardware limits compared with deterministic, retrieval, small-model, and larger-model alternatives."
domains:
- "agent"
- "hardware"
- "research"
- "software"
- "system"
aliases: []
relations:
- type: "tests"
  target: "GONI-PRINCIPLE-9062425CD490"
- type: "tests"
  target: "GONI-DECISION-84F201170733"
- type: "derived_from"
  target: "EVID-SOUP2026-LOWVRAM"
sources:
- "SRC-SOUP-REPO"
- "SRC-SOUP-BENCHMARKS"
- "SRC-HU2022-LORA"
- "SRC-FRUGALGPT2023"
artifacts: []
uncertainty: "This is a proposed protocol with no GONI measurements. The usefulness of fine-tuning, Soup performance, hardware support, calibration, maintenance burden, and amortization volume must be established empirically before recommending a runtime profile."
legacy: []
---

# EXP-ADAPT-01 — Local model specialization

## Research question

For recurring GONI tasks, when does a locally specialized model deliver adequate intelligence at lower *total* cost than existing software and model alternatives? Separate two questions: **(1)** whether learned adaptation adds value beyond deterministic logic, retrieval, and inference routing; and **(2)** if adaptation adds value, whether Soup's low-VRAM execution is the most suitable training method on the available hardware.

This is a blueprint experiment, not an execution report or a decision to adopt Soup.

## Workloads and evaluation data

Select a bounded, frequent, consented workload such as inbox intent triage, structured extraction, or drafting assistance. Define the expected output schema, permissible evidence, acceptable error severity, abstention behavior, and downstream handoff. Keep policy enforcement in the GONI kernel regardless of model choice.

Create a source-traceable, de-duplicated dataset with training, validation, and held-out test partitions separated by case/entity and, where relevant, time. Include difficult and rare cases, missing information, ambiguous inputs, prompt-injection attempts, and distribution shifts. Record labeling procedure, human adjudication, rights to use each datum for training, and revocation/deletion behavior.

## Comparative baselines

Compare complete task pipelines under matched quality and provenance requirements:

1. Deterministic rules, validation, and reusable cached computation.
2. A lightweight classifier or embedding-based / Jeff-like decision model, with measured probability calibration.
3. A small resident quantized model with task-specific prompts and retrieval, without weight updates.
4. A small model trained with conventional LoRA/QLoRA where supported.
5. A matched base model with Soup layer streaming and comparable LoRA targets, data, precision, and effective optimization budget.
6. A more capable reference model with the same evaluation inputs and GONI-compliant egress and authorization.

Treat the *pipeline* comparison separately from a **training-method A/B test** of resident/offloaded versus streamed adaptation. A smaller model may win the pipeline comparison even if Soup wins the memory-capacity comparison.

## Reproducibility and hardware controls

Pin source-repository commit, training-tool version, model/base revision and hash, tokenizer/template, adapters, quantization scheme, dataset version, seeds, optimizer, LoRA targets/rank, training tokens, batch, sequence length, checkpointing, and inference backend. Run initial short synthetic checks before any substantial training expenditure.

Use separate hardware-result tables for discrete CUDA GPUs and GONI's target unified-memory APU. Record unsupported paths explicitly. Constrain CPU and accelerator memory, disk and NVMe usage, wall time, power, thermal load, and background interference with interactive jobs.

## Measures

- **Task quality:** held-out correctness, schema validity, severity-weighted errors, abstention quality, calibration error, and reviewer acceptance.
- **Router impact:** escalation frequency, quality lost relative to a reference route, and measured regret under the existing small-then-big routing decision.
- **Training and deployment:** peak GPU and system memory, bytes read/written, tokens/s, elapsed time, energy where measured, failure rate, artifact size, startup time, and adapter reload integrity.
- **Operational value:** p50/p95 response latency, steady-state energy and cost per accepted task, quality drift, retraining frequency, maintenance effort, and interactions with other GONI jobs.
- **Governance:** dataset access provenance, adapter lineage and version pinning, approval trace, egress compliance, and unauthorized side effects.

Evaluate for a declared task volume `N` using a total-cost model:

```text
C_total(N) = C_data + C_training + C_validation + C_deployment
           + N * C_inference + C_maintenance + C_failures_and_rework
```

Present measured amounts separately from estimates and scenario assumptions. Report comparative cost at more than one plausible volume rather than assuming that training expense is always amortized.

## Acceptance and promotion gates

1. **Load-bearing adaptation:** confirm nonzero adapter updates, successful save/reload into the intended inference runtime, and reproducible performance on a frozen held-out evaluation set.
2. **Numerical compatibility:** compare streamed and resident paths using the *same quantized checkpoint* where feasible; specify exact-equality or tolerance-based checks explicitly. Test actual trained-artifact behavior, not only step-zero logits.
3. **Quality and economics:** demonstrate a predeclared improvement or noninferiority at a lower verified full-lifecycle cost relative to the strongest practical simpler baseline; quantify uncertainty and severe-failure regressions.
4. **Hardware and scheduling:** pass measured memory, energy, latency, thermal, storage, and concurrent-interactive-work limits on the *intended* device. A 4 GB CUDA demonstration does not qualify an APU backend.
5. **Authority and data governance:** retain model/kernel separation; require authorized data usage, revision-pinned model registration, ordinary tool mediation, network gating, and receipts for consequential deployment changes.

If a gate fails, preserve the negative result and prefer the existing cheaper or safer task route. A training backend is a replaceable component; it acquires no policy, memory, or execution authority.

## Implementation boundary

The runnable harness, fixtures, and results belong in [goni-prototype-lab](https://github.com/duracell04/goni-prototype-lab). The blueprint remains `status: draft` and `implementation_state: not_applicable`. This experiment changes no kernel contract, model-registry schema, deployed dependency, or hardware decision.
