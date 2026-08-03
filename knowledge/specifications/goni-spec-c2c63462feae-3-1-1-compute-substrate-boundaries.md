---
id: GONI-SPEC-C2C63462FEAE
title: 3.1.1 Compute substrate boundaries
type: specification
status: draft
implementation_state: specified_only
proposition: Goni MVP hardware targets local AI inference and adjacent classical AI workloads.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 3.1.1 Compute substrate boundaries
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 3.1.1 Compute substrate boundaries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1.1 Compute substrate boundaries

Goni MVP hardware targets local AI inference and adjacent classical AI
workloads. AI hardware accelerates tensor and matrix operations through CPUs,
GPUs, TPUs, NPUs, high-bandwidth memory, fast interconnects, memory locality,
compiler/runtime optimization, and energy-efficient scheduling.

Quantum hardware is a different physical computation substrate. It preserves
and transforms fragile quantum states through qubit control, isolation from
noise, measurement systems, error correction, and classical control
electronics. A quantum processor MUST NOT be treated as a drop-in accelerator
for the MVP local AI runtime.

The MVP platform requirements therefore remain focused on unified memory,
sustained bandwidth, thermal stability, storage endurance, telemetry, and
runtime backend readiness. Future external compute or offload APIs may remain
substrate-agnostic, but they MUST NOT introduce a quantum dependency into the
MVP hardware baseline.
