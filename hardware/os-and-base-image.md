# OS and Base Image (Hardware Contract)

Last refreshed: **2026-01-03**

This document defines the base-image telemetry and capability discovery
requirements for Goni hardware. It is a pure refactor from the supplier map.

## 1. Telemetry and capability discovery (base image contract)

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

## 2. Failure modes and fallbacks

If telemetry is incomplete, the system MUST default to conservative routing:

- prefer CPU/iGPU paths,
- reduce solver duty cycle,
- defer compaction and index maintenance.
