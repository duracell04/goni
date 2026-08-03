---
id: GONI-SYNTHESIS-C3A53AF60D52
title: 6. Privacy, data routing, and keys
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Data path: only the prompt/context needed for the task goes to cloud; keep local artifacts local where possible.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 6. Privacy, data routing, and keys
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 6. Privacy, data routing, and keys

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Privacy, data routing, and keys
- Data path: only the prompt/context needed for the task goes to cloud; keep local artifacts local where possible.
- Redaction: orchestrator should strip direct identifiers or use summaries when feasible before cloud send.
- Keys: stored locally (encrypted config); council use is opt-in and requires explicit configuration.
- Raw private or sensitive context is not eligible for default Council routing.
  The router must either keep the task local, send a public/redacted payload, or
  require an explicit approval corridor.
