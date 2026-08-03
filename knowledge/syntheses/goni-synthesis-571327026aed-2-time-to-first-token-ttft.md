---
id: GONI-SYNTHESIS-571327026AED
title: 2) Time to first token (TTFT)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Time until the model produces its first generated token.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: 2) Time to first token (TTFT)
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# 2) Time to first token (TTFT)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2) Time to first token (TTFT)

Time until the model produces its first generated token. It includes:
- queue wait (if any)
- prompt assembly (system + user + context)
- prefill compute
- first decode step

For interactive UX, TTFT is usually the single most important latency metric.
