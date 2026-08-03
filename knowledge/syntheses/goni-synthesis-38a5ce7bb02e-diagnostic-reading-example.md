---
id: GONI-SYNTHESIS-38A5CE7BB02E
title: Diagnostic reading example
type: synthesis
status: draft
implementation_state: specified_only
proposition: If TTFC and TTFT are fast but "first summary token" is late, the system is likely spending time in retrieval/tools or deliberately deferring the summary until evidence is gathered.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: Diagnostic reading example
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# Diagnostic reading example

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Diagnostic reading example

If TTFC and TTFT are fast but "first summary token" is late, the system is
likely spending time in retrieval/tools or deliberately deferring the summary
until evidence is gathered. If total time is very long after the summary
begins, generation is slow or tools are still running.
