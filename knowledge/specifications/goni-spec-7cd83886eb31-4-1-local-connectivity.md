---
id: GONI-SPEC-7CD83886EB31
title: 4.1 Local Connectivity
type: specification
status: draft
implementation_state: specified_only
proposition: 'The device must provide: At least one **wired network port** (Ethernet).'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 4.1 Local Connectivity
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 4.1 Local Connectivity

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.1 Local Connectivity

- The device must provide:
  - At least one **wired network port** (Ethernet).
  - Wireless connectivity (Wi-Fi) for initial setup and optional operation.

- Wired networking should:
  - Support **higher-than-gigabit throughput** (for example, multi-gigabit speeds) to enable efficient communication between multiple nodes and fast transfers to/from local infrastructure (NAS, routers, etc.).
