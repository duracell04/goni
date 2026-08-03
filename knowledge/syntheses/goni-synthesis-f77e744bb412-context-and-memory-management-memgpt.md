---
id: GONI-SYNTHESIS-F77E744BB412
title: Context and memory management (MemGPT)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'MemGPT (Packer et al., 2023) frames LLM context as **virtual memory**: prompt window ≈ RAM, external stores ≈ Disk, with explicit paging calls.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-whitepaper.md
  heading: Context and memory management (MemGPT)
  revision: 66b954ceb474004d6304fd1fb280804bae3e7e6b
---

# Context and memory management (MemGPT)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Context and memory management (MemGPT)

MemGPT (Packer et al., 2023) frames LLM context as **virtual memory**: prompt window ≈ RAM, external stores ≈ Disk, with explicit paging calls. Goni internalises this in the planes: Memory/Context planes page Arrow/ANN/graph stores into context, LLM engines remain stateless, and paging/syscalls (e.g. `MEM_READ`, `MEM_WRITE`) are first-class kernel APIs rather than prompt tricks.
