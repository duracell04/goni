# Goni Hardware – Overview (MVP)

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

We are intentionally **technology-agnostic** here:  
no specific CPU/GPU brands are required in this file. What matters are the behaviours and constraints.

---

## 1. What the Goni box should be

At a high level, the Goni hardware should:

- **Look and feel** like a high-end, minimalistic device:
  - compact rectangular box (roughly 6–8 L volume),
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
  - train lightweight adapters (e.g. LoRA-style) on personal data.

- **Act as a cluster node**:
  - multiple Goni boxes on a network should combine into one logical AI system,
  - nothing beyond normal networking hardware should be required for a small cluster (2–4 nodes).

---

## 2. What this folder contains

- [`10-requirements.md`](./10-requirements.md)  
  The primary reference for **hardware requirements** (size, power, connectivity, noise, serviceability, future-proofing).  
  If you want to propose a different enclosure, power profile, or base platform, start by checking and updating this file.

- `20-architecture-options.md`  
  (To be filled) Design options and trade-offs: e.g. APU-only vs APU+GPU, different layout concepts, etc.

- `30-mechanical/`  
  Enclosure concepts, airflow notes, and eventually CAD files.

- `40-electronics/`  
  Power distribution, front-panel MCU, status LEDs, any custom boards or harnesses.

- `50-bom-experiments/`  
  Bill-of-materials experiments and cost models, comparing different component choices that satisfy the requirements.

- `90-decisions.md`  
  Accepted hardware decisions (ADR-style). Each decision should reference relevant discussions and requirement changes.

---

## 3. How to contribute (hardware)

1. Read [`10-requirements.md`](./10-requirements.md) to understand the current constraints.
2. If you propose a change that affects those constraints:
   - open a hardware issue,
   - and suggest concrete edits to `10-requirements.md`.
3. For new designs or experiments:
   - add them to `20-architecture-options.md` or a new file under `30-mechanical/` or `40-electronics/`,
   - link them from an issue or pull request.
4. For decisions that are accepted, add an entry to `90-decisions.md`.

The aim is to converge on a **buildable, testable Goni node** that meets the shared requirements, not just an idealised spec.

For detailed discussion of candidate hardware architectures and example products, see [20-architecture-options.md](./20-architecture-options.md).
