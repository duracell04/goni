---
id: GONI-SPEC-5819106486B4
title: 3.1 Compute Capability
type: specification
status: draft
implementation_state: specified_only
proposition: 'The hardware must be capable of: Running **medium-to-large language models** locally with interactive latency for a single user, and acceptable latency for a small group.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 3.1 Compute Capability
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 3.1 Compute Capability

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Compute Capability

The hardware must be capable of:

- Running **medium-to-large language models** locally with interactive latency for a single user, and acceptable latency for a small group.
- Supporting **fine-tuning of adapters** (e.g. LoRA-style) on such models using personal data.
- Handling **multiple concurrent tasks**:
  - ongoing background indexing of documents and emails,
  - serving chat / coding assistants,
  - running lightweight agents.

We do **not** require:

- Full, from-scratch training of very large models on-device.
- Matching the throughput of data-center GPU servers.
