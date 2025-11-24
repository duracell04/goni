# Goni Hardware Requirements (MVP)

> This document describes what the **final Goni product** should look like and be able to do, from a hardware and physical-system perspective.  
> It is deliberately technology-agnostic (no specific brands, chips, or vendors).

---

## 1. Purpose

Goni is a **small, local-first AI node** designed to live in a home or office and:

- run a powerful personal AI assistant **on-device**,
- host the user’s own data and indices,
- mesh with additional nodes to scale out,
- optionally attach to more powerful external accelerators in the future.

All hardware decisions should support this purpose.

---

## 2. Form Factor & Physical Design

### 2.1 Size & Shape

- The device should be a **compact, standalone box**, not a rack server or tower PC.
- Target external volume: **approximately 6–8 litres**.
- Rough size envelope (for discussion, not final):  
  - height: 20–30 cm  
  - depth: 20–25 cm  
  - width: 10–15 cm  

### 2.2 Aesthetics

- Appearance should be **minimal, neutral, and “living room safe”**:
  - Matte or satin surface, preferably dark (e.g. black / dark grey).
  - No RGB lighting, no gamer styling, no visible branding beyond a small mark.
- Front panel elements:
  - **One power button**.
  - **One slender status light bar** (multi-state but visually calm).

### 2.3 User-Facing Touchpoints

- The box must support:
  - Safe **power on/off** by a non-technical user.
  - Clear visual feedback via the light bar:
    - normal / ready
    - busy / processing
    - updating
    - fault / overheat
- No keyboard, mouse, or display should be required for normal use after initial provisioning.

---

## 3. Performance & Capacity Requirements

### 3.1 Compute Capability

The hardware must be capable of:

- Running **medium-to-large language models** locally with interactive latency for a single user, and acceptable latency for a small group.
- Supporting **fine-tuning of adapters** (e.g. LoRA-style) on such models using personal data.
- Handling **multiple concurrent tasks**:
  - ongoing background indexing of documents and emails,
  - serving chat / coding assistants,
  - running lightweight agents.

We do **not** require:

- Full, from-scratch training of very large models on-device.
- Matching the throughput of data-center GPU servers.

### 3.2 Memory & Storage

The system must:

- Provide enough **system memory** and/or **unified memory** to:
  - host at least one medium-to-large model (tens of billions of parameters) in compressed form,
  - maintain in-memory indices and caches for personal data,
  - run supporting services without constant swapping.

- Provide persistent **storage** for:
  - operating system and base software,
  - model files for multiple models,
  - embeddings / indices,
  - user data and configuration.

Storage requirements should assume:

- Multi-terabyte local storage as a baseline.
- At least **one expansion path** (e.g. an extra internal slot) for future capacity increases.

### 3.3 Power Envelope

- The system should operate comfortably from a **standard household power outlet**.
- Target sustained draw under heavy AI workloads: **on the order of a few hundred watts**, not kilowatts.
- Peak draw should be handled by the power subsystem without instability or audible stress.

---

## 4. Connectivity & Mesh Capability

### 4.1 Local Connectivity

- The device must provide:
  - At least one **wired network port** (Ethernet).
  - Wireless connectivity (Wi-Fi) for initial setup and optional operation.

- Wired networking should:
  - Support **higher-than-gigabit throughput** (for example, multi-gigabit speeds) to enable efficient communication between multiple nodes and fast transfers to/from local infrastructure (NAS, routers, etc.).

### 4.2 Mesh / Multi-Node Operation

- Goni must be designed as a **first-class cluster node**:
  - It should be straightforward to run multiple devices on the same network and treat them as a **single logical AI cluster**.
- No special external hardware beyond standard networking equipment should be required for a small mesh (2–4 nodes).
- Latency-sensitive workloads should work well when the user interacts with **any one** of the nodes in the mesh.

---

## 5. Reliability, Serviceability, Lifecycle

### 5.1 Reliability

- The device is expected to run **24/7** under varying load.
- Thermal design must ensure:
  - Components stay within safe temperature limits under sustained heavy AI workloads.
  - Ambient room temperatures typical of homes/offices are supported without throttling or instability.

### 5.2 Acoustic Behaviour

- Under light and typical workloads (chat, coding, indexing), the device should be:
  - Subjectively **“near silent”** in a quiet room.
- Under heavy workloads (adapter training, long-running jobs), fan noise is acceptable but:
  - Should remain within **reasonable desktop/workstation levels**,
  - Should not sound like a “server room” or high-RPM blower.

### 5.3 Serviceability

- Users or technicians should be able to:
  - Access internal components with basic tools (or a simple latch) for maintenance and replacement (e.g. storage, fans, power supply, main board if possible).
  - Clean or replace **dust filters** without opening the entire device.

- The design should aim for:
  - Reasonable **component modularity**,
  - Clear identification of replaceable parts.

---

## 6. Security & Physical Safety

- The enclosure must:
  - Protect users from contact with live electrical components.
  - Comply with relevant safety expectations for consumer electronics (temperature of external surfaces, stability on a desk, etc.).

- Hardware features should support:
  - Optional **physical security** (e.g. Kensington or similar anchor point),
  - Safe operation in shared office environments.

---

## 7. Future-Proofing Expectations

The hardware design should anticipate:

- That compute boards / accelerators may be **swapped or upgraded** over the product’s lifetime without redesigning the entire enclosure and power domain.
- That external **“heavy nodes”** (e.g. more powerful accelerators in separate devices) can be attached over the network in future, and Goni should be ready from a power and networking standpoint.

The constraints in this document should remain valid even as specific chip generations change.

---

## 8. Explicit Non-Goals (Hardware MVP)

For the MVP, Goni is **not** intended to be:

- A multi-rack data-center solution.
- A general-purpose gaming PC.
- A battery-powered portable device.
- A trivially passively cooled / fanless device (under realistic performance requirements).

These may be explored as separate product lines, but are out of scope for the hardware MVP.

