---
id: GONI-SPEC-C90B69DCA9AD
title: 5.1 Reliability
type: specification
status: draft
implementation_state: specified_only
proposition: The device is expected to run **24/7** under varying load.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 5.1 Reliability
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 5.1 Reliability

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.1 Reliability

- The device is expected to run **24/7** under varying load.
- Thermal design must ensure:
  - Components stay within safe temperature limits under sustained heavy AI workloads.
  - Ambient room temperatures typical of homes/offices are supported without throttling or instability.
