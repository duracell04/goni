---
id: GONI-OBJECTION-16BE2D441BA6
title: 2. Failure modes and fallbacks
type: objection
status: draft
implementation_state: not_applicable
proposition: 'If telemetry is incomplete, the system MUST default to conservative routing: prefer CPU/iGPU paths, reduce solver duty cycle, defer compaction and index maintenance.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/os-and-base-image.md
  heading: 2. Failure modes and fallbacks
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 2. Failure modes and fallbacks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Failure modes and fallbacks

If telemetry is incomplete, the system MUST default to conservative routing:

- prefer CPU/iGPU paths,
- reduce solver duty cycle,
- defer compaction and index maintenance.
