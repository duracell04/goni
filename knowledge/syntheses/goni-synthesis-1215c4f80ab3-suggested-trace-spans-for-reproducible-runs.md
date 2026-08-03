---
id: GONI-SYNTHESIS-1215C4F80AB3
title: Suggested trace spans (for reproducible runs)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'ingress policy_check context_assemble prefill decode_stream tool_calls[] (nested) egress[] (network gate decisions + bytes) receipt_write complete Derived metrics: TTFC = first output in decode_stream (or earlier if heartbeat) TTFT = first token in decode_stream first summary token = first token after summary_section_start marker'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: Suggested trace spans (for reproducible runs)
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# Suggested trace spans (for reproducible runs)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Suggested trace spans (for reproducible runs)

1) ingress
2) policy_check
3) context_assemble
4) prefill
5) decode_stream
6) tool_calls[] (nested)
7) egress[] (network gate decisions + bytes)
8) receipt_write
9) complete

Derived metrics:
- TTFC = first output in decode_stream (or earlier if heartbeat)
- TTFT = first token in decode_stream
- first summary token = first token after summary_section_start marker
- response time = complete
