---
id: GONI-SYNTHESIS-2BC1189E68DA
title: 4) Response time
type: synthesis
status: draft
implementation_state: specified_only
proposition: Total time until the system reports completion (stream closed / final output).
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: 4) Response time
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# 4) Response time

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4) Response time

Total time until the system reports completion (stream closed / final output).

Long tails after summary starts typically mean:
- summary is only one section and more content follows, or
- generation is slow (low tokens/sec), or
- tools are still running / waiting / throttled.
