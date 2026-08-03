---
id: GONI-SYNTHESIS-64FA9E8D697F
title: Config and ops hooks
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Seats/weights and triggers: goni-prototype-lab:config/council.yaml (ground truth).'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/remote-llm-architecture.md
  heading: Config and ops hooks
  revision: 4fc11a4a1fff204c88ed6df6a2bacd84bc6453ce
---

# Config and ops hooks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Config and ops hooks
- Seats/weights and triggers: `goni-prototype-lab:config/council.yaml` (ground truth).
- Env defaults for the council service: see `goni-prototype-lab:config/council.env.example` (`OPENROUTER_API_KEY`, `OPENROUTER_MODEL`, `OPENROUTER_URL`, `MAX_REMOTE_TOKENS_PER_DAY`, `ALLOW_REMOTE_TOOLS`, `REMOTE_MODE`, optional `OPENROUTER_HTTP_REFERER`/`OPENROUTER_X_TITLE`).
- Web/search tools live on the council side; Goni OS never ships a cloud browser tool in the base box.
