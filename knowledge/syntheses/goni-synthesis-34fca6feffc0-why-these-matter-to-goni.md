---
id: GONI-SYNTHESIS-34FCA6FEFFC0
title: Why these matter to Goni
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'First chunk: "Did the system respond at all?" (liveness) First token: "It''s alive and not stuck." (perceived responsiveness) First summary token: "It delivered the useful part." (value latency) Total time: "How long until it''s fully done?" (completion latency) For an operator appliance, value latency (time to first actionable output)'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: Why these matter to Goni
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# Why these matter to Goni

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Why these matter to Goni

- First chunk: "Did the system respond at all?" (liveness)
- First token: "It's alive and not stuck." (perceived responsiveness)
- First summary token: "It delivered the useful part." (value latency)
- Total time: "How long until it's fully done?" (completion latency)

For an operator appliance, value latency (time to first actionable output)
matters as much as TTFT.
