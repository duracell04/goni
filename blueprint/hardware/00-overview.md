# Goni Hardware - Overview (MVP)

Last refreshed: **2026-01-03**

This folder contains everything related to the **physical Goni node**:

- the box you put on a desk or shelf,
- the components inside it,
- how it behaves in the real world (noise, thermals, reliability),
- and how multiple boxes can work together as a small cluster.

The goal of the hardware work is to define and build a **small, quiet, robust AI appliance** that:

- runs a powerful personal AI assistant locally,
- can stay on 24/7 in a normal home or office,
- can be combined with other Goni nodes over the network,
- can evolve over the next hardware generations without redesigning everything.

We stay **technology-agnostic at the product level**, but we still converge on concrete **reference SKUs** for the MVP so that mechanical/electrical/software can be validated end-to-end.

---

## 1. What the Goni box should be

At a high level, the Goni hardware should:

- **Look and feel** like a high-end, minimalistic device:
  - compact rectangular box (target: ~7 L, allowed: 6???8 L),
  - matte, neutral finish (e.g. black / dark grey),
  - one power button and one calm status light bar on the front,
  - no RGB, no gamer aesthetics.

- **Fit into real homes and offices**:
  - small enough to sit next to a router, speaker, or monitor,
  - quiet enough to live under or on a desk,
  - no special power requirements (standard wall outlet).

- **Deliver enough performance** to:
  - run medium-to-large language models locally with interactive latency,
  - index and search through a personal document / email corpus,
  - handle multiple tasks in parallel (chat, indexing, small jobs),
  - train lightweight adapters (LoRA-style) on personal data.

- **Act as a cluster node**:
  - multiple Goni boxes on a network should combine into one logical AI system,
  - nothing beyond normal networking hardware should be required for a small cluster (2???4 nodes).

---

## 1.1 Physical constraints that define local ITCR

This section encodes the hardware consequences of inference-time compute
reasoning (ITCR) as platform constraints and interfaces, not performance claims.

### 1.1.1 Memory wall (roofline framing)

Define arithmetic intensity as:

I = FLOPs / Byte.

Decoding/generation tends to be low-I and thus memory-bound; prefill/encoding
can be more compute-bound. As a result, platform selection MUST prioritize
sustained bandwidth, latency stability, and memory residency over peak TOPS.
See `blueprint/hardware/appendix/roofline.md` for the roofline primer.

### 1.1.2 Two-regime power model

Goni operates in two regimes:

- Continuous cognition (always-on encoders + predictor).
- Reasoning bursts (solver/ITCR interrupts).

The product requirement is to minimize solver duty cycle while preserving burst
responsiveness without destabilizing thermals. Hardware MUST support stable
operation under both regimes, with telemetry to distinguish them.

### 1.1.3 Memory topology

UMA reduces copies but shares bandwidth; a discrete GPU provides dedicated VRAM
bandwidth but incurs PCIe transfer and wake penalties.

Normative requirement:
- UMA is preferred for high-frequency state exchange.
- dGPU is acceptable only if state shuttling over PCIe is avoided.

### 1.1.4 Supported minimum vs reference platform

Hardware SHOULD define a supported minimum and a reference platform for
validation. This file must remain qualitative; quantitative targets live in
`blueprint/hardware/90-decisions.md` and are updated only with evidence.

### 1.1.5 Traceability map (signals to actions)

Telemetry signal -> scheduler decision -> runtime routing -> persistence action:

- memory pressure -> shrink context / route to higher bandwidth path
- thermal/DVFS state -> clamp burst duty cycle
- accelerator shape support -> route to compatible device
- storage endurance -> gate compaction and index maintenance

Cross-layer links:
- hardware constraints -> `blueprint/software/10-requirements.md`
- runtime routing -> `blueprint/software/30-components/llm-runtime.md`
- duty cycle/hysteresis -> `blueprint/30-specs/scheduler-and-interrupts.md`

### 1.1.6 Failure modes and fallbacks

The platform MUST expose enough signals to detect and mitigate:

- memory-bound stall and tail-latency spikes,
- thermal runaway and prolonged throttling,
- swap thrash from oversubscription,
- write amplification during compaction.

Fallbacks include routing to CPU/iGPU, lowering duty cycle, and deferring
background compaction until safe conditions return.

---

## 2. What this folder contains

- [`10-requirements.md`](/blueprint/hardware/10-requirements.md)  
  The primary reference for **hardware requirements** (size, power, connectivity, noise, serviceability, future-proofing).

- [`20-architecture-options.md`](/blueprint/hardware/20-architecture-options.md)  
  Updated survey of **candidate architectures and concrete 2025/2026 SKUs**, with an explicit MVP recommendation and resolved open questions.

- [`25-hardware-layers-and-supplier-map.md`](/blueprint/hardware/25-hardware-layers-and-supplier-map.md)  
  Opinionated map of **accelerator/supplier landscape**, **availability**, and how it aligns to Goni tiers and current software backend readiness.
- [`os-and-base-image.md`](/blueprint/hardware/os-and-base-image.md)  
  OS-level telemetry and capability discovery contract for the base image.

- [`30-mechanical/`](/blueprint/hardware/30-mechanical)  
  Enclosure concepts, airflow notes, and draft thermal + acoustic plans.

- [`40-electronics/`](/blueprint/hardware/40-electronics)  
  Power distribution assumptions, PSU choices, front-panel MCU, LED/status bar, harnessing.

- [`50-bom-experiments/`](/blueprint/hardware/50-bom-experiments)  
  Bill-of-materials experiments and component snapshots. New versions should be added rather than overwriting older ones.

- [`90-decisions.md`](/blueprint/hardware/90-decisions.md)  
  Accepted hardware decisions (ADR-style): baseline architecture, networking, enclosure envelope, PSU approach, etc.
- [`appendix/roofline.md`](/blueprint/hardware/appendix/roofline.md)  
  Roofline primer used by ITCR platform contracts.

---

## 3. Current MVP reference (so everything can be validated)

See [`90-decisions.md`](/blueprint/hardware/90-decisions.md) for the canonical decisions, but in one line:

- **MVP reference compute module:** APU-centric node based on a **Ryzen AI Max+ 395 class** board with **128 GB unified LPDDR5X** (Framework Desktop mainboard is the primary reference; HP Z2 Mini G1a is the off-the-shelf fallback).  
- **Supported minimum:** 64 GB unified-memory devices may be used for early development and testing, but they are not performance-representative for the product story and must be treated as a degraded mode (see `blueprint/software/10-requirements.md`).
- **MVP enclosure envelope:** ~7 L, quiet, front status bar, internal SFX PSU, 2?? NVMe.  
- **MVP networking:** 5 GbE preferred (2.5 GbE acceptable only as fallback for early dev boxes).

---

## 4. How to contribute (hardware)

1. Read [`10-requirements.md`](/blueprint/hardware/10-requirements.md) to understand constraints.
2. If you propose a change that affects constraints:
   - open a hardware issue, and
   - suggest concrete edits to [`10-requirements.md`](/blueprint/hardware/10-requirements.md).
3. For new designs or experiments:
   - add them to [`20-architecture-options.md`](/blueprint/hardware/20-architecture-options.md) or under `30-mechanical/` / `40-electronics/`,
   - link them from an issue or pull request.
4. For accepted decisions:
   - add or update an ADR entry in [`90-decisions.md`](/blueprint/hardware/90-decisions.md).

The aim is to converge on a **buildable, testable Goni node** that meets shared requirements, not just an idealised spec. Cross-check software constraints in `blueprint/software/` (LLM runtime backends, networking, storage layout) so the box and stack stay aligned.


