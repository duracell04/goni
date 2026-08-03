---
id: GONI-SPEC-97DBAF405880
title: 3.5 ITCR Platform Requirements
type: specification
status: draft
implementation_state: specified_only
proposition: This section defines platform contracts for inference-time compute reasoning (ITCR).
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 3.5 ITCR Platform Requirements
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 3.5 ITCR Platform Requirements

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.5 ITCR Platform Requirements

This section defines platform contracts for inference-time compute reasoning
(ITCR). These are hardware-level requirements that enable the software policies
defined in `blueprint/software/10-requirements.md`, `blueprint/software/30-components/llm-runtime.md`,
and `blueprint/30-specs/scheduler-and-interrupts.md`.

#### 3.5.1 Platform signals (telemetry requirements)

The platform MUST expose, or the OS MUST provide a documented fallback for:

- Memory pressure indicators (free RAM, swap activity, major page faults).
- Thermal sensors and throttling events (per domain where available).
- DVFS state (current perf state/clock domain indicators where possible).
- Storage endurance signals (SMART/health where possible; at least bytes written).
- Accelerator capability query (NPU supported ops/graphs, max tensor shapes, supported quantization).
- GPU wake/active state (at least active vs idle and approximate residency).

These signals are required inputs to the scheduler and routing policies.

#### 3.5.2 Platform knobs (control requirements)

The OS SHOULD be able to enforce:

- Reasoner duty-cycle control (rate limits, cooldown, hysteresis).
- DVFS clamping policy for burst compute (operate near the efficiency frontier).
- NPU fixed-shape bucketing (token bucket sizes; fixed image/audio shapes) with deterministic fallback to CPU/iGPU.
- Write budgeting and compaction gating (only compact/index under safe power/thermal conditions).
- Pinned/shared memory regions for hot state (avoid paging kernel state).

#### 3.5.3 Write amplification control

Write amplification factor (WAF) is the ratio of physical writes to logical
writes. The platform MUST support an endurance-safe persistence model:

- LSM-style buffering or equivalent (memtable/ring buffer -> sequential flush segments -> gated compaction).
- Vector index maintenance MUST be deferred by policy and MUST NOT be triggered
  by high-frequency state updates.

#### 3.5.4 Memory topology requirement

- UMA path: zero-copy state exchange is preferred.
- dGPU path: avoid PCIe state shuttling; keep continuous loops on CPU/NPU/iGPU,
  use dGPU only for bursts.

Cross-link: KV cache paging/segmentation and shape routing live in
`blueprint/software/30-components/llm-runtime.md`.

#### 3.5.5 Failure modes and fallbacks

The platform MUST support fallbacks for:

- thermal runaway (clamp duty cycle, defer background work),
- swap thrash (reduce context and cache pressure),
- compaction interference (pause writes under burst load),
- accelerator shape mismatch (route to CPU/iGPU).


---
