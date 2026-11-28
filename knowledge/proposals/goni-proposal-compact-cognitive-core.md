---
id: GONI-PROPOSAL-COMPACT-COGNITIVE-CORE
title: Compact cognitive core and selective internalization
type: proposal
status: draft
implementation_state: specified_only
proposition: "Goni should evaluate compact reasoning-oriented models as replaceable cognitive components, internalizing reusable capabilities while retrieving mutable facts and delegating exact computation."
domains:
- agent
- research
- software
- system
aliases: []
relations:
- type: refines
  target: GONI-PRINCIPLE-9062425CD490
- type: supports
  target: GONI-DECISION-84F201170733
- type: derived_from
  target: GONI-SYNTHESIS-502B238AFDE1
sources:
- SRC-KARPATHY-DWARKESH-2025
- SRC-HINTON-DISTILLATION-2015
- SRC-DEEPSEEK-R1-2025
- SRC-SHUMAILOV-COLLAPSE-2024
artifacts: []
uncertainty: "The minimum sufficient model size, separability of knowledge and reasoning, and end-to-end cost advantage are empirical questions. No Goni implementation or benchmark is asserted."
legacy: []
---

# Compact cognitive core and selective internalization

## Research hypothesis

Karpathy's cognitive-core intuition suggests that next-token pretraining allocates capacity to both reusable abstractions and arbitrary factual associations. A compact model trained on curated curricula and teacher demonstrations might retain useful reasoning and tool-selection capabilities while relying on external knowledge and deterministic computation. The one-billion-parameter scale is a conjecture, not a requirement or demonstrated threshold. A cryptographic hash is an illustrative case: computing it with a verified algorithm is preferable to relying on memorized associations.

Knowledge and reasoning are distributed across weights; there is no established clean partition. Internal linguistic, semantic and procedural knowledge remains necessary. Distillation transfers selected capabilities through teacher distributions or generated demonstrations; quantization changes numerical representation, pruning removes components, and retrieval supplies information at inference. These mechanisms are complementary.

## Relationship to the existing canon

- SYS-04 describes the human-centred cognitive exocortex as a non-normative explanatory model. The compact core is a replaceable component, not the human principal or a new authority boundary.
- GONI-PRINCIPLE-9062425CD490 already specifies large addressable capacity with a small dynamically selected active state. Selective internalization extends the allocation question to training and model selection, distinct from runtime working-set selection.
- D-009 already specifies small-then-big routing with a regret budget. The compact-core hypothesis supplies candidate models for that policy rather than replacing it.
- EVID-ROUTE-01 already proposes evaluating frugal sovereign routing. A compact-core comparison can extend that evaluation.
- GONI-SYNTHESIS-502B238AFDE1 already tracks Karpathy as a design signal, not a normative source.

## Architecture and economics

The model supplies probabilistic interpretation, abstraction, planning and tool selection. External state supplies provenance-bearing facts, episodic memory and procedural artifacts. Deterministic algorithms execute exact operations; specialist models and larger models provide additional capabilities on demand. Existing kernel contracts retain authority, permissions, state transitions and verification. Model outputs are proposals subject to those contracts.

Distinguish first-order model capability, second-order orchestration and third-order improvement of allocation policies from observed outcomes. The latter two are Goni design interpretations, not claims attributable to Karpathy. A JEV-style router is a candidate for cheap allocation decisions; its suitability requires independent measurement.

Minimize expected end-to-end cost subject to task quality, privacy, authority and consequence constraints. Include retrieval, verification, escalation, latency, energy, local hardware, and failure/rework costs. A cheap model is valuable only when the complete execution path remains sufficiently reliable.

## Research basis and limitations

Hinton et al. establish teacher-student distillation; DeepSeek-R1 reports transfer of selected reasoning capabilities into smaller models, not general intelligence at a fixed parameter count. Shumailov et al. demonstrate risks of recursive synthetic-data training. Curated synthetic curricula require grounding, diversity, verification and held-out evaluation. Karpathy's interview is a hypothesis-generating source, not experimental proof.

## Proposed evaluation

Compare a small model alone, a large model alone, and a compact model with governed retrieval, deterministic tools and escalation under matched tasks and explicit budgets. Measure task success, calibration, false all-clears, escalation rate, regret, provenance completeness, policy violations, latency, energy and total cost. Include unfamiliar tasks, stale facts, rare high-consequence evidence and retrieval/tool failures. Preserve existing authority boundaries and use the existing routing evaluation lane. Record model versions, hardware, prompts, datasets and pinned test evidence before any implementation or status promotion.
