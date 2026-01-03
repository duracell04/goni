# 25 - Hardware Layers and Supplier Map (2025–2026)

Last refreshed: **2026-01-03**

This note captures the **mental model for Goni hardware**, plus an opinionated supplier map with **availability reality** and **backend readiness** notes so hardware choices stay aligned with what the software can actually run today.

---

## 1. Layered mental model

Everything maps to four layers:

- **Silicon / Accelerators** — GPUs, NPUs, ASICs, photonic processors; what inference backends ultimately target.
- **Systems / Boxes** — workstations, servers, mini PCs; what sits on a desk or in a rack.
- **Edge / Always-On** — ultra-low-power inference for router/intent models that keep big compute asleep.
- **Workloads / Models** — what drives specs (LLMs, vision, embeddings, diffusion, adapters).

---

## 2. Goni tiers (v1 / Pro / Max)

### v1 – APU node / quiet appliance (MVP baseline)

Goal: quiet, compact, always-on personal exocortex.

- Compute: APU with **128 GB unified memory** (Ryzen AI Max+ 395 class).
- Box: custom ~7 L enclosure (internal SFX PSU, 2× NVMe, 5 GbE).
- Priority suppliers: **Framework (mainboard), HP (fallback dev box), AMD (APU platform)**.

Availability snapshot:
- Framework Desktop mainboard (Ryzen AI Max+ 395 / 128 GB) is orderable in batches on Framework Marketplace.
- HP Z2 Mini G1a 128 GB configs are listed in Switzerland via HP store and aggregators.

### Pro – Single big GPU exocortex

Goal: maximum compatibility + throughput for CUDA-first stacks (vLLM, diffusion, etc.).

- Compute: 1× NVIDIA RTX-class GPU (24–32 GB VRAM), desktop CPU, 64–128 GB DDR5.
- Box: tower or larger SFF (acoustics become harder).
- Priority suppliers: **NVIDIA (default), EU integrators**.

### Max – Cluster / experimental / future accelerators

Goal: “AI lab” setups: multi-GPU, alternative accelerators, future photonics.

- Compute: multi-GPU (NVIDIA HGX-class) and/or alt accelerators (Gaudi, Tenstorrent).
- Systems: Supermicro-class chassis and integrators.
- Track (not MVP): photonic and optical IO/fabric suppliers.

---

## 3. Backend readiness (software ↔ hardware alignment)

This section prevents “paper hardware” that the current stack can’t use.

### 3.1 What the repo can run today (kernel reality)

In `software/kernel/goni-infer`, the implemented engine is **HttpVllmEngine** (a client to a vLLM server).
That means:

- **MVP end-to-end today is easiest on NVIDIA/CUDA**, because vLLM is most mature there.
- **APU-centric MVP requires either:**
  - validating vLLM on the target AMD APU/ROCm stack, or
  - adding a second runtime backend (recommended) such as **llama.cpp** (Vulkan/HIP/CPU fallback).

Actionable implication:
- Hardware MVP can still be APU-centric, but we must track the “runtime gap” explicitly and close it on the software side.

### 3.2 Practical runtime options by tier

| Tier | Hardware target | “Works now” runtime path | Notes |
|------|------------------|--------------------------|-------|
| v1 (APU) | Ryzen AI Max+ 395 class | llama.cpp (Vulkan/HIP) or validated ROCm path | vLLM ROCm officially targets specific AMD GPUs; APU validation is a concrete task, not an assumption. |
| Pro (NVIDIA) | RTX 4090/5090 class | vLLM (CUDA) | Best current throughput and ecosystem maturity. |
| Max | multi-GPU / mixed | vLLM + custom | Requires more orchestration (sharding, networking, scheduling). |

### 3.3 Telemetry and capability discovery (base image contract)

The base image MUST expose, or provide a documented fallback for:

- thermal sensors and throttling events,
- memory pressure and swap statistics,
- storage writes and health signals,
- GPU/NPU capability query (supported shapes, quantization, graph cache status),
- optional bandwidth estimates or perf counters where available.

OS policies MUST support:

- background compaction/indexing only on AC power and with thermal headroom,
- pausing background work during solver bursts,
- pinning shared memory regions for hot state.

Cross-layer links:
- scheduling behavior: `software/10-requirements.md`
- routing and shape constraints: `software/30-components/llm-runtime.md`

### 3.4 Failure modes and fallbacks

If telemetry is incomplete, the system MUST default to conservative routing:

- prefer CPU/iGPU paths,
- reduce solver duty cycle,
- defer compaction and index maintenance.

---

## 4. Silicon / accelerator suppliers (prioritised)

### 4.1 Tier-1 (near-term, real)

- **AMD (APU platform)** — critical for v1 APU-centric box. Track ROCm/NPU tooling maturity.
- **NVIDIA (dGPU)** — default for Pro and many Max deployments due to CUDA ecosystem.
- **Intel (Gaudi / client NPUs)** — interesting for Max/alt vendor strategy; not an MVP dependency.

### 4.2 Tier-2 (alternative accelerators)

- **Tenstorrent** — developer-friendly alternative backend; good long-term hedge if it becomes practical.
- **Groq / Cerebras / SambaNova** — mostly “as-a-system” or DC-class; not MVP.

### 4.3 Tier-0 (future / photonics)

- **Arago, Lightmatter, Q.ANT, Scintil** — track for Max-only future; keep out of MVP critical path.

---

## 5. Systems / box suppliers (availability + where to lean)

- **APU mini-node sources (v1):** Framework (board), HP (Z2 Mini G1a), plus mini-PC vendors as yardsticks.
- **EU-friendly integrators (Pro/Max):** AIME / BIZON / Supermicro resellers, etc.
- **OEM workstations:** Dell / Lenovo / HPE (enterprise-friendly, but less “modular appliance”).

---

## 6. How to use this map

- Use this file to keep tiers **opinionated** and tied to **backend reality**.
- When a new backend lands (e.g. llama.cpp runtime), update the readiness table and then revisit what hardware becomes “unblocked”.
- Keep the MVP path boring and buildable; move experimental suppliers into Pro/Max until they are proven.
