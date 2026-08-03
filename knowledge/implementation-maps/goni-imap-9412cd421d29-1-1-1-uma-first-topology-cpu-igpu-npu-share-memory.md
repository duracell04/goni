---
id: GONI-IMAP-9412CD421D29
title: 1.1.1 UMA-first topology (CPU + iGPU + NPU share memory)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Benefits: Fewer copies and more stable latency for frequent state access.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: 1.1.1 UMA-first topology (CPU + iGPU + NPU share memory)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 1.1.1 UMA-first topology (CPU + iGPU + NPU share memory)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.1.1 UMA-first topology (CPU + iGPU + NPU share memory)

Benefits:
- Fewer copies and more stable latency for frequent state access.

Risks:
- Bandwidth contention and thermal coupling between CPU/iGPU/NPU.

Invariants:
- Hot state stays resident in shared memory.
- Solver bursts MUST NOT trigger large host-device copies.

Routing implications:
- Prefer UMA for memory-bound decoding and frequent state exchange.

Telemetry needs:
- Memory bandwidth/pressure signals and DVFS state per domain.

Failure modes and fallbacks:
- Bandwidth contention -> throttle bursts and reduce background work.
- Thermal coupling -> clamp duty cycle and defer compaction.
