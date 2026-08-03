---
id: GONI-IMAP-73166F728A6D
title: 3.1 External API (MVP)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'HTTP /v1/chat/completions: Request: compatible with OpenAI’s chat/completions: messages[], model, stream, optional ools.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/orchestrator.md
  heading: 3.1 External API (MVP)
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 3.1 External API (MVP)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 External API (MVP)

HTTP /v1/chat/completions:

- Request: compatible with OpenAI’s chat/completions:
  - messages[], model, stream, optional 	ools.
- Response:
  - Non-streaming or server-sent events with tokens.
