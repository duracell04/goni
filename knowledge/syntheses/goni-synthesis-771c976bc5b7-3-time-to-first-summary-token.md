---
id: GONI-SYNTHESIS-771C976BC5B7
title: 3) Time to first summary token
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'A product-specific marker: "first token that belongs to the summary section." It implies a phased pipeline, for example: Phase A: quick acknowledgement Phase B: retrieval/tool calls / heavy reasoning Phase C: summary output begins If this is late relative to TTFT, it usually indicates retrieval/tools or intentional deferral of the "useful" output until evidence is gathered.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: 3) Time to first summary token
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# 3) Time to first summary token

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3) Time to first summary token

A product-specific marker: "first token that belongs to the summary section."
It implies a phased pipeline, for example:
- Phase A: quick acknowledgement
- Phase B: retrieval/tool calls / heavy reasoning
- Phase C: summary output begins

If this is late relative to TTFT, it usually indicates retrieval/tools or
intentional deferral of the "useful" output until evidence is gathered.
