---
id: COG-JOINT-01
title: Process-first heterogeneous cognitive joints
type: proposal
status: draft
implementation_state: specified_only
proposition: A governed Goni workflow may allocate bounded semantic judgments to independently replaceable
  heterogeneous cognitive components at typed decision points, while the deterministic kernel retains state,
  policy and effect authority.
domains:
- agent
- software
- hardware
- kernel
- research
aliases: []
relations:
- type: refines
  target: GONI-IMAP-216164F1995B
  note: Interpret the existing Harness Plane as a governed operational topology rather than assume an LLM-centric
    loop.
- type: refines
  target: GONI-DECISION-1A482B12FFB8
  note: Distinguish routine local judgment from budgeted deliberative interrupts.
- type: refines
  target: GONI-DECISION-84F201170733
  note: Generalize small/large allocation across heterogeneous judgment mechanisms.
- type: depends_on
  target: GONI-DECISION-9AF49466170D
  note: Kernel-owned authority stays separate from learned cognition.
- type: depends_on
  target: GONI-DECISION-7CD481DA15B7
  note: Essential operation remains local and offline-capable.
- type: depends_on
  target: INF-PLACE-01
  note: Resource-feasible hardware and model placement remains a governed concern.
sources:
- SRC-BROOKS1991-SUBSUMPTION
- SRC-SUTTON1999-OPTIONS
- SRC-BAIR2024-COMPOUND
- SRC-CHEN2025-LLMSELECTOR
- SRC-TYPESAFE2026-JUDGMENT
- SRC-BELCAK2025-SLM-AGENTS
artifacts: []
uncertainty: This is an architectural research proposal, not a new kernel contract or evidence of Goni's implementation.
  Benefit, decomposition granularity, interfaces, local-model availability, calibration, and maintenance remain
  unmeasured.
legacy: []
---

# Process-first heterogeneous cognitive joints

**Research date:** 2026-11-27.

## Design proposition

The operational body of a delegated computation is its typed state, events,
work-order objective, permissible state transitions, and execution interfaces.
Its cognitive joints are independently measurable points where the next
valid transition depends on interpretation, estimation, prediction, or a
choice under uncertainty. Intelligence is distributed throughout those points.
A general LLM is available for ambiguous, novel, compositional, or generative
work where it earns its computational cost.

The architectural inversion concerns the **primary unit of composition**:
a workflow and its decision contracts precede model allocation. It extends
the existing Harness Plane and agents-as-local-processes stance. It leaves
the formal node tuple, authority model, and existing decisions unchanged.
A single capable model remains a legitimate implementation where decomposition
offers insufficient benefit.

| Body function | Candidate implementation | Output |
| --- | --- | --- |
| Skeleton and connective tissue | Typed workflow/state machine, event transport | State/version references and admissible transitions |
| Senses | OCR, speech, vision encoder, embedding model | Bounded observations with provenance |
| Reflex | Exact rule, lookup, calibrated classifier or decision-native model | Predicate, label, score, abstention |
| Joint | Task-specific policy, trained ranker, solver, small model | Ranked candidate among permitted next steps |
| Anticipation | Forecast, action-conditioned world model where validated | Predicted outcome and uncertainty |
| Deliberation | General LLM or other planner | Proposed plan, explanation, candidate composition |
| Muscle | Deterministic, capability-mediated application operation | Receipt or explicit failure |

A classic classifier, gradient-boosted tree, constraint solver, discriminative
neural model, typed System One model, SLM and frontier LLM represent distinct
computational mechanisms. Assign a mechanism to the question, rather than
require every joint to generate language. A JEPA-like world model is a
prediction candidate, not a synonym for a decision-native model such as Jev.

## Illustrative contract

A joint consumes a versioned task contract, minimally sufficient authorized
state/evidence, declared candidate outcomes, a resource budget, and provenance.
It returns a typed judgment over those candidates, an uncertainty or abstention
indicator where supported, model/profile identification, and timing/resource
measurements. Its prediction is advisory; the kernel checks freshness,
applicability, permissions and the current authoritative state and either
accepts a valid transition, escalates, or declines.

A typed response proves shape conformance only. It establishes neither truth
nor permission. Calibration is measured for each workload and version; confidence
is not itself evidence that supplied state is current. Constitutive constraints,
arithmetic, authoritative identity, exact membership, validation, effect
permissions, budgets, revocation, and commits remain deterministic controls.

Essential local/offline behavior must retain an available local path or explicit
abstention. A hosted Jev-like API is an optional comparator or policy-approved
nonessential cognitive service; it is not a required Goni dependency. Data
minimization, principal authorization and existing egress controls apply.

## Cross-layer allocation

The same logical contract can be evaluated on a CPU, GPU, NPU, or a supported
specialized accelerator appropriate to its compute, memory, latency, power and
lifecycle profile. A model identity and substrate are selected together with
the task requirements, subject to the existing placement and budget contracts;
changes to either are versioned and evaluated. A fixed model in silicon is a
future placement alternative for mature stable workloads, not a new MVP
hardware requirement.
