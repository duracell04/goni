---
id: SYN-COG-JOINT-01
title: 'Hardware, model and harness co-design: intelligence at the joints'
type: synthesis
status: draft
implementation_state: not_applicable
proposition: The hardware–model specialization analogy connects Goni's existing harness, routing, local-first
  and authority principles to a research program in process-first heterogeneous cognition and conditional lifecycle
  economics.
domains:
- hardware
- software
- agent
- research
- market
aliases: []
relations:
- type: synthesizes
  target: COG-JOINT-01
  note: Process-first cognitive topology and joint contract.
- type: synthesizes
  target: COG-SPEC-01
  note: Specialization, allocation and business economics.
- type: synthesizes
  target: EVID-TAALAS-202611
  note: External model-specific silicon precedent.
- type: synthesizes
  target: EVID-JEV-202611
  note: External decision-native judgment precedent.
- type: synthesizes
  target: OBJ-COG-JOINT-01
  note: Dependency and uncertainty objections.
- type: synthesizes
  target: EXP-COG-JOINT-01
  note: Proposed comparative evaluation.
sources:
- SRC-BROOKS1991-SUBSUMPTION
- SRC-SUTTON1999-OPTIONS
- SRC-LECUN2022-JEPA
- SRC-ASSRAN2025-VJEPA2
- SRC-BAIR2024-COMPOUND
- SRC-KHATTAB2024-DSPY
- SRC-CHEN2025-LLMSELECTOR
- SRC-BELCAK2025-SLM-AGENTS
- SRC-HSIEH2023-DISTILLATION
- SRC-ROZINAT2006-DECISION-MINING
artifacts: []
uncertainty: Research synthesis and architectural analogy only. No claim of novel invention, operational improvement,
  local Jev availability, market demand, or Goni implementation is established.
legacy: []
---

# Hardware, model and harness co-design: intelligence at the joints

## Research origin (27 November 2026)

Taalas's specialized model-in-silicon approach suggests a broader question:
could a software delegation system be designed around the body's individual
decision points, using a different computational mechanism where appropriate,
rather than assume that a universal language model must control the whole body?
The analogy is architectural and economic, not a claim of identical
physical mechanisms or the need to manufacture chips for Goni.

| Layer | General-purpose relationship | Specialization hypothesis |
| --- | --- | --- |
| Hardware ↔ model | Load compatible parameters into general compute and memory | Co-design parameters and compute/data movement for a stable model |
| Model ↔ harness | A general LLM interprets context and orchestrates operations | A typed operational graph allocates cognition to its individual joints |
| Software ↔ hardware | Choose compute platform for an aggregate model workload | Choose a feasible model, runtime and placement for each required joint |

On each layer, specialization exchanges breadth and upgrade flexibility for
potentially improved marginal cost, latency, memory and energy. The optimum
is workload dependent. Typed software boundaries, selective escalation and
versioned components preserve more substitution flexibility than fixed
silicon. They also create coordination and validation costs.

## Research lineage

| Tradition | Relevance |
| --- | --- |
| Brooks (1991), behavior-based robotics | Independent reactive activity producers illustrate decentralized computation. |
| Sutton, Precup and Singh (1999), hierarchical RL | Temporally extended policies separate actions at different abstraction levels. |
| LeCun (2022), JEPA; Assran et al. (2025), V-JEPA 2 | Predictive latent world models illustrate a non-language-based component; physical robotics evaluations remain domain-specific. |
| Rozinat and van der Aalst (2006), decision mining | Derive candidate decision points from process event records instead of an imagined workflow. |
| Zaharia et al. (2024), compound AI | Multiple model, tool and retrieval components can form a single engineered system. |
| Khattab et al. (2024), DSPy | Declarative language-model programs can be optimized against defined metrics. |
| Chen et al. (2025), LLMSelector | Different participating modules can benefit from different model allocations. |
| Hsieh et al. (2023), distillation; Belcak et al. (2025), agentic SLMs | Repeated tasks motivate task-specific student models when measured economics and quality justify conversion. |

Jev and JEPA are distinct. Jev is a commercially exposed typed judgment
interface for narrowly specified choices, score and Boolean probability
questions. JEPA is a predictive representation-learning family. An
architecture may also contain classical models, exact rules and optimization
algorithms; the heterogeneity is the research question.

## Reading map

- [COG-JOINT-01](../proposals/cog-joint-01-heterogeneous-cognitive-joints.md)
  specifies the proposed body/joints decomposition without modifying existing Goni authority contracts.
- [EVID-TAALAS-202611](../evidence/evid-taalas-202611-model-specific-silicon.md)
  and [EVID-JEV-202611](../evidence/evid-jev-202611-typed-system-one-judgments.md)
  distinguish source reports, estimates, technical limitations and their possible architectural implications.
- [OBJ-COG-JOINT-01](../objections/obj-cog-joint-01-cascading-and-inherited-judgment-error.md)
  identifies inherited-state error and the cost of unsafe decomposition.
- [COG-SPEC-01](../proposals/cog-spec-01-cognitive-specialization-economics.md)
  describes the specialization strategy, break-even logic and commercialization hypotheses.
- [EXP-COG-JOINT-01](../experiments/exp-cog-joint-01-cross-layer-cognitive-architecture-evaluation.md)
  states a falsifiable comparison and the evidence required for promotion.

This extends the existing blueprint rather than replacing the kernel,
formal planes, model placement contract, local-first requirement, or
probabilistic-cognition/deterministic-authority decision. All proposed
mechanisms remain specified only until bounded experiments produce
reviewable evidence.
