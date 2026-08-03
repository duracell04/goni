---
id: GONI-IMAP-FBBFE14FE1A3
title: 1. Telemetry and capability discovery (base image contract)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The base image MUST expose, or provide a documented fallback for: thermal sensors and throttling events, memory pressure and swap statistics, storage writes and health signals, GPU/NPU capability query (supported shapes, quantization, graph cache status), optional bandwidth estimates or perf counters where available.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/os-and-base-image.md
  heading: 1. Telemetry and capability discovery (base image contract)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 1. Telemetry and capability discovery (base image contract)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
- scheduling behavior: `blueprint/software/10-requirements.md`
- routing and shape constraints: `blueprint/software/30-components/llm-runtime.md`
