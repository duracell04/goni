---
id: GONI-SPEC-A221A420C8C4
title: 2. Metering dimensions (normative)
type: specification
status: draft
implementation_state: specified_only
proposition: 'For each execution, the runtime MUST capture where applicable: tokens_in, tokens_out latency_ms tool_calls bytes_egress ram_peak_bytes'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-01-execution-metering.md
  heading: 2. Metering dimensions (normative)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 2. Metering dimensions (normative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Metering dimensions (normative)
For each execution, the runtime MUST capture where applicable:
- `tokens_in`, `tokens_out`
- `latency_ms`
- `tool_calls`
- `bytes_egress`
- `ram_peak_bytes`
