---
id: GONI-SYNTHESIS-239696D47DEE
title: Determinism and self-loop stability
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Autoregressive self-loops are chaotic: tiny numeric noise can flip token choices over long runs.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-whitepaper.md
  heading: Determinism and self-loop stability
  revision: 66b954ceb474004d6304fd1fb280804bae3e7e6b
---

# Determinism and self-loop stability

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Determinism and self-loop stability

Autoregressive self-loops are chaotic: tiny numeric noise can flip token choices over long runs. For regulated or auditable workflows, Goni exposes a **deterministic preset** (temperature 0, fixed seed where supported, batch size 1, single worker/thread or CPU, TF32 off on NVIDIA, deterministic backend flag such as vLLM `--enable-deterministic-inference`). Hardware/driver hashes are logged with deterministic runs so trajectories can be reproduced, even if throughput is lower.
