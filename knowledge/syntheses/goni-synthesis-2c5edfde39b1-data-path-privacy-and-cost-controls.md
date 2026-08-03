---
id: GONI-SYNTHESIS-2C5EDFDE39B1
title: Data path, privacy, and cost controls
type: synthesis
status: draft
implementation_state: specified_only
proposition: Trim/sanitise context before sending; prefer summaries or extracted facts instead of raw artifacts.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/remote-llm-architecture.md
  heading: Data path, privacy, and cost controls
  revision: 4fc11a4a1fff204c88ed6df6a2bacd84bc6453ce
---

# Data path, privacy, and cost controls

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Data path, privacy, and cost controls
- Trim/sanitise context before sending; prefer summaries or extracted facts instead of raw artifacts.
- Enforce per-call and per-day budgets (tokens or $) and drop to local-only when exceeded or unavailable.
- Log every remote call (Arrow table) with model/provider, usage/cost, latency, and decision outcome; keep request/response snippets minimal for privacy.
- Raw private, legal/financial/identity-sensitive, or confidential context
  remains local by default. Remote routing requires a public-only payload,
  redaction, or an explicit approval corridor.
