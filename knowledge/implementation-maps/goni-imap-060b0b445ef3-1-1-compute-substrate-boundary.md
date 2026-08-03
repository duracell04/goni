---
id: GONI-IMAP-060B0B445EF3
title: 1.1 Compute substrate boundary
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The Goni MVP is a classical AI appliance.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 1.1 Compute substrate boundary
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 1.1 Compute substrate boundary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1.1 Compute substrate boundary

The Goni MVP is a classical AI appliance. Its hardware accelerates local
inference, embeddings, retrieval, indexing, adapter tuning, and burst reasoning
through conventional compute substrates: CPU, GPU, NPU, memory hierarchy,
storage, interconnects, and runtime/compiler support.

Quantum computing is out of scope for MVP hardware. It is a different
engineering problem: preserving and transforming quantum states with qubit
control, isolation, measurement, error correction, and classical control
electronics. Quantum processors may matter later for specific mathematical
subproblems, but they are not stronger AI accelerators and are not part of the
Goni baseline.

---
