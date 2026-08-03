---
id: GONI-IMAP-346C9CB21145
title: 1.2.5 Traceability map (signals to actions)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Telemetry signal -> scheduler decision -> runtime routing -> persistence action: memory pressure -> shrink context / route to higher bandwidth path thermal/DVFS state -> clamp burst duty cycle accelerator shape support -> route to compatible device storage endurance -> gate compaction and index maintenance Cross-layer links:'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 1.2.5 Traceability map (signals to actions)
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 1.2.5 Traceability map (signals to actions)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1.2.5 Traceability map (signals to actions)

Telemetry signal -> scheduler decision -> runtime routing -> persistence action:

- memory pressure -> shrink context / route to higher bandwidth path
- thermal/DVFS state -> clamp burst duty cycle
- accelerator shape support -> route to compatible device
- storage endurance -> gate compaction and index maintenance

Cross-layer links:
- hardware constraints -> `blueprint/software/10-requirements.md`
- runtime routing -> `blueprint/software/30-components/llm-runtime.md`
- duty cycle/hysteresis -> `blueprint/30-specs/scheduler-and-interrupts.md`
