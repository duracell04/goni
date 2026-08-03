---
id: GONI-PRINCIPLE-99769DA756A5
title: 4. Invariants & performance targets
type: principle
status: draft
implementation_state: specified_only
proposition: '**Capability invariant** ModelCapabilities must approximate real behaviour well enough that ??''s scheduling assumptions (capacity region) are not violated.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/llm-runtime.md
  heading: 4. Invariants & performance targets
  revision: 6ce37ef5d3a676fd26377a3fa8a15c5b226016c2
---

# 4. Invariants & performance targets

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Invariants & performance targets

* **Capability invariant**
  ModelCapabilities must approximate real behaviour well enough that ??'s scheduling assumptions (capacity region) are not violated.

* **Bundle immutability invariant**
  The runtime executes a declared bundle ID and must not mutate trunk weights,
  expert deltas, router rules, or retrieval config on the hot path.

* **Three-speed invariant**
  Fresh facts belong in retrieval or memory inputs, domain skill belongs in
  scoped modules, and core weights change only by loading a new promoted
  bundle.

* **Shape-compatibility invariant**
  If a request's shape falls outside the backend's supported buckets, the runtime
  must route to a compatible device (CPU/iGPU/GPU) rather than padding or
  recompiling on the hot path.

* **Budget safety invariant**
  generate must not exceed max_tokens without explicit override.

* **Wake latency invariant**
  Time-to-first-token after a decoder wake must stay within the configured SLO; steady-state operation must not trigger implicit compilation or graph warmup.

* **Preemption invariant (soft)**
  Generation checks for cancellation at least once per decoding step (target preemption latency « human-visible 100 ms).

* **Streaming invariant**
  First token latency for interactive jobs stays within configured SLO (e.g. p99 < 1.0 s on reference hardware).

* **Deterministic preset**
  Runtime exposes a deterministic profile for audit/self-loop runs: temperature 0, fixed seed when backend supports it, batch size 1, no continuous/dynamic batching, single worker/thread (or CPU-only fallback), TF32 disabled on NVIDIA. Backend flags must be set accordingly (e.g. vLLM `--enable-deterministic-inference`; llama.cpp/Ollama single slot/thread), and the runtime logs device + driver hashes with each deterministic run.

---
