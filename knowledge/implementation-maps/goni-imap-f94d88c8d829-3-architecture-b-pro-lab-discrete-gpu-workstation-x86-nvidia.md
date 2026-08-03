---
id: GONI-IMAP-F94D88C8D829
title: '3. Architecture B (Pro / Lab): discrete GPU workstation (x86 + NVIDIA dGPU)'
type: implementation-map
status: draft
implementation_state: specified_only
proposition: This architecture is ideal for ???Pro???
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: '3. Architecture B (Pro / Lab): discrete GPU workstation (x86 + NVIDIA dGPU)'
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3. Architecture B (Pro / Lab): discrete GPU workstation (x86 + NVIDIA dGPU)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Architecture B (Pro / Lab): discrete GPU workstation (x86 + NVIDIA dGPU)

This architecture is ideal for ???Pro??? tiers and lab workloads, but it conflicts with the MVP envelope:

- louder and larger in practice,
- much higher power budget,
- bigger thermal design burden.

Still, it is the cleanest path to ???CUDA-first??? toolchains and vLLM throughput.
