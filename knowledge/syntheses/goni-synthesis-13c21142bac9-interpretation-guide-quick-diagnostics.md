---
id: GONI-SYNTHESIS-13C21142BAC9
title: Interpretation guide (quick diagnostics)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Good TTFC and TTFT with slow TTFST usually means tool or retrieval latency.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics.md
  heading: Interpretation guide (quick diagnostics)
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# Interpretation guide (quick diagnostics)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Interpretation guide (quick diagnostics)
- Good TTFC and TTFT with slow TTFST usually means tool or retrieval latency.
- Long tail after summary start can indicate slow decode or heavy post-summary work.
- Large token gaps often feel worse than steady slow output.
