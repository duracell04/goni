---
id: GONI-SPEC-7AFF49DE66D2
title: 4.1 Controlled External Calls
type: specification
status: draft
implementation_state: specified_only
proposition: 'The system may use external AI services to: handle tasks that are too difficult or large for local models, obtain web search results or live data, provide a second opinion on critical content.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 4.1 Controlled External Calls
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 4.1 Controlled External Calls

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.1 Controlled External Calls

- The system may use external AI services to:
  - handle tasks that are too difficult or large for local models,
  - obtain web search results or live data,
  - provide a second opinion on critical content.

- All such calls must be:
  - **Optional** – users can disable cloud usage entirely.
  - **Transparent** – it should be clear when an external service is used.
  - **Budgeted** – the system must enforce configurable ceilings on external usage.
