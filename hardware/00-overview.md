# Goni Hardware - Overview (MVP)

Last refreshed: **2025-12-14**

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
  - compact rectangular box (target: ~7 L, allowed: 6–8 L),
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
  - nothing beyond normal networking hardware should be required for a small cluster (2–4 nodes).

---

## 2. What this folder contains

- [`10-requirements.md`](./10-requirements.md)  
  The primary reference for **hardware requirements** (size, power, connectivity, noise, serviceability, future-proofing).

- [`20-architecture-options.md`](./20-architecture-options.md)  
  Updated survey of **candidate architectures and concrete 2025/2026 SKUs**, with an explicit MVP recommendation and resolved open questions.

- [`25-hardware-layers-and-supplier-map.md`](./25-hardware-layers-and-supplier-map.md)  
  Opinionated map of **accelerator/supplier landscape**, **availability**, and how it aligns to Goni tiers and current software backend readiness.

- [`30-mechanical/`](./30-mechanical/)  
  Enclosure concepts, airflow notes, and draft thermal + acoustic plans.

- [`40-electronics/`](./40-electronics/)  
  Power distribution assumptions, PSU choices, front-panel MCU, LED/status bar, harnessing.

- [`50-bom-experiments/`](./50-bom-experiments/)  
  Bill-of-materials experiments and cost snapshots. New versions should be added rather than overwriting older ones.

- [`90-decisions.md`](./90-decisions.md)  
  Accepted hardware decisions (ADR-style): baseline architecture, networking, enclosure envelope, PSU approach, etc.

---

## 3. Current MVP reference (so everything can be validated)

See [`90-decisions.md`](./90-decisions.md) for the canonical decisions, but in one line:

- **MVP reference compute module:** APU-centric node based on a **Ryzen AI Max+ 395 class** board with **128 GB unified LPDDR5X** (Framework Desktop mainboard is the primary reference; HP Z2 Mini G1a is the off-the-shelf fallback).  
- **MVP enclosure envelope:** ~7 L, quiet, front status bar, internal SFX PSU, 2× NVMe.  
- **MVP networking:** 5 GbE preferred (2.5 GbE acceptable only as fallback for early dev boxes).

---

## 4. How to contribute (hardware)

1. Read [`10-requirements.md`](./10-requirements.md) to understand constraints.
2. If you propose a change that affects constraints:
   - open a hardware issue, and
   - suggest concrete edits to [`10-requirements.md`](./10-requirements.md).
3. For new designs or experiments:
   - add them to [`20-architecture-options.md`](./20-architecture-options.md) or under `30-mechanical/` / `40-electronics/`,
   - link them from an issue or pull request.
4. For accepted decisions:
   - add or update an ADR entry in [`90-decisions.md`](./90-decisions.md).

The aim is to converge on a **buildable, testable Goni node** that meets shared requirements, not just an idealised spec. Cross-check software constraints in `software/` (LLM runtime backends, networking, storage layout) so the box and stack stay aligned.
