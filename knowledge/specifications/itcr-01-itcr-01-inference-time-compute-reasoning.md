---
id: ITCR-01
title: ITCR-01 - Inference-Time Compute Reasoning
type: specification
status: draft
implementation_state: specified_only
proposition: "\uFEFF--- id: ITCR-01 type: SPEC status: specified_only DOC-ID: ITCR-01 Status: Specified only / roadmap This spec defines inference-time compute reasoning (ITCR) as a bounded, interrupt-driven reasoning service."
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/itcr.md
  heading: ITCR-01 - Inference-Time Compute Reasoning
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# ITCR-01 - Inference-Time Compute Reasoning

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# ITCR-01 - Inference-Time Compute Reasoning
﻿---
id: ITCR-01
type: SPEC
status: specified_only
---
DOC-ID: ITCR-01
Status: Specified only / roadmap

This spec defines inference-time compute reasoning (ITCR) as a bounded,
interrupt-driven reasoning service. ITCR is a compute allocation policy over a
reasoning procedure, not a model architecture. It trades extra latency/energy
for reduced error on multi-step planning, verification, and constraint-heavy
tasks.
