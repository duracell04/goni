---
id: GONI-DECISION-4B7FAADA7D85
title: Context
type: decision
status: draft
implementation_state: specified_only
proposition: The current kernel inference engine in-repo is HTTP vLLM client.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/90-decisions.md
  heading: Context
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Context

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Context

The current kernel inference engine in-repo is HTTP vLLM client. vLLM is mature on CUDA/NVIDIA and has ROCm support for some AMD GPUs, but APU-class support must be validated (or a second backend must be added).
