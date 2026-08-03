---
id: GONI-IMAP-1578E14145F3
title: 1. Role in the system
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The OS + base image layer provides the **execution substrate** for a Goni node.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/os-and-base-image.md
  heading: 1. Role in the system
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 1. Role in the system

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Role in the system

The OS + base image layer provides the **execution substrate** for a Goni node. It is not part of the kernel (??, ??, ??, ??), but defines:

- how goni-node / goni-http runs as a long-lived service,
- how CPU/GPU/NPU resources are exposed to the LLM runtime (??),
- where persistent state for the Arrow Spine (??) and models lives.

We treat this layer as a **black box with minimal assumptions**, so the kernel stays portable across distros/containers.

---
