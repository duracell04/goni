---
id: GONI-SPEC-409F6720C3CD
title: 6.4 Observability Without Surveillance
type: specification
status: draft
implementation_state: specified_only
proposition: 'Logs should: be sufficient to diagnose issues and track resource usage, avoid storing unnecessary sensitive content in plain text.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 6.4 Observability Without Surveillance
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 6.4 Observability Without Surveillance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.4 Observability Without Surveillance

- Logs should:
  - be sufficient to diagnose issues and track resource usage,
  - avoid storing unnecessary sensitive content in plain text.
- Receipts and durable memory provenance must remain under Goni-kernel control
  even when external adapters or agent gateways are present.

---
