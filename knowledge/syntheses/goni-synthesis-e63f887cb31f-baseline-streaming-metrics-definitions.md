---
id: GONI-SYNTHESIS-E63F887CB31F
title: Baseline streaming metrics (definitions)
type: synthesis
status: draft
implementation_state: specified_only
proposition: The following four metrics should be pinned to explicit pipeline events.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics.md
  heading: Baseline streaming metrics (definitions)
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# Baseline streaming metrics (definitions)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Baseline streaming metrics (definitions)
The following four metrics should be pinned to explicit pipeline events.

| Metric | Definition | Event marker |
| --- | --- | --- |
| Time to first chunk (TTFC) | Time from client send to first streamed bytes reaching the client. | First SSE/stream frame emitted, even if it is a heartbeat. |
| Time to first token (TTFT) | Time from client send to the first model-generated token. | First token event in `decode_stream`. |
| Time to first summary token (TTFST) | Time from client send to the first token that belongs to the summary section. | First token after an explicit `summary_section_start` marker. |
| Response time | Time from client send to stream completion. | Stream end or `complete` event. |

Notes:
- TTFC can be a liveness signal, not proof that model compute started.
- TTFST requires a deterministic marker emitted by the app, not model heuristics.
