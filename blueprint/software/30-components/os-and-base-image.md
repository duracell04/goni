# 10 – OS and Base Image

Status: MVP – conceptual substrate  
Scope: Single-node execution environment assumptions

---

## 1. Role in the system

The OS + base image layer provides the **execution substrate** for a Goni node. It is not part of the kernel (??, ??, ??, ??), but defines:

- how goni-node / goni-http runs as a long-lived service,
- how CPU/GPU/NPU resources are exposed to the LLM runtime (??),
- where persistent state for the Arrow Spine (??) and models lives.

We treat this layer as a **black box with minimal assumptions**, so the kernel stays portable across distros/containers.

---

## 2. Responsibilities & boundaries (MVP)

### 2.1 Responsibilities

For the MVP, the substrate is responsible for:

- **Process model**
  - Start goni-http (or goni-node) on boot (systemd, Docker, k8s, …).
  - Capture logs via stdout/stderr (journald or container logs).

- **Resource exposure**
  - Make CPU cores available to Rust/Arrow/Wasm.
  - Optionally expose GPU/NPU devices to the LLM runtime (via CUDA/ROCm/Metal/NPUs).


- **Memory and device hygiene**
  - Provide a way to pin/lock latency-critical pages (latent state, encoder buffers).
  - Allow swap to be disabled or encrypted for state pages.
  - Enable IOMMU or equivalent DMA protection when available.

- **Storage**
  - Provide *persistent* directories for:
    - model weights (e.g. /opt/goni/models),
    - data plane (e.g. /var/lib/goni for indices, metrics),
    - configuration (e.g. /etc/goni or $XDG_CONFIG_HOME/goni).

### 2.2 Non-responsibilities

Out of scope for the MVP:

- Custom “Goni OS” distribution or immutable image.
- Automated disk encryption / secure boot story.
- Cluster bootstrap and node join (lives in blueprint/hardware/infra docs).

---

## 3. Interfaces & assumptions

The kernel assumes:

- A POSIX-like environment with:
  - a writable, durable directory for data,
  - a writable directory for models,
  - a writable temp directory.
- A process supervisor that can:
  - start the Goni service,
  - restart on failure,
  - expose environment variables for configuration.

We **do not** fix whether Goni runs on bare metal, in a VM, or in a container; the component spec is deliberately deployment-agnostic.

---

## 4. Invariants (from 30-conformance)

- **Local-first invariant**  
  Core operation (chat + RAG with local models) must be possible with *no* external network connectivity.

- **Durability invariant**  
  Reboot does not lose:
  - model files,
  - indices/embeddings,
  - configuration.


- **State hygiene invariant**
  Latent state pages must not be swapped in plaintext, and crash dumps must redact or exclude state buffers.

- **Isolation invariant**  
  Default permissions ensure Goni data directories are not world-readable; GPU/NPU access is restricted to the Goni service user where applicable.

---

## 5. MVP vs future “appliance”

**MVP / prototype**

- Run on any modern 64-bit Linux.
- Dependencies (Rust toolchain, model runtimes) installed manually or via container.
- No opinionated disk layout beyond “persistent dirs exist”.

**Future appliance**

- Curated base image(s) and hardware targets.
- Encrypted volumes for models/data.
- Preconfigured systemd units / k8s manifests with health checks and auto-update strategy.

Those details live in blueprint/hardware/ and infra repos; this file records the *software-visible OS assumptions* only.



## 6. Upstream
- [Hardware requirements](/blueprint/software/hardware/10-requirements.md)
- [Isolation and tool sandboxes](/blueprint/30-specs/isolation-and-tool-sandboxes.md)

## 7. Downstream
- [LLM runtime](/blueprint/software/30-components/llm-runtime.md)
- [Mesh and WireGuard](/blueprint/software/30-components/mesh-and-wireguard.md)

## 8. Adjacent
- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Security overview](/blueprint/software/security/00-overview.md)

