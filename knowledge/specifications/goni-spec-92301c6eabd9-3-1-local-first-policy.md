---
id: GONI-SPEC-92301C6EABD9
title: 3.1 Local-First Policy
type: specification
status: draft
implementation_state: specified_only
proposition: 'The system should prefer **local model inference** whenever reasonably possible, for: chat and reasoning, summarisation, coding help, RAG responses.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 3.1 Local-First Policy
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.1 Local-First Policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Local-First Policy

- The system should prefer **local model inference** whenever reasonably possible, for:
  - chat and reasoning,
  - summarisation,
  - coding help,
  - RAG responses.

- Local models should be:
  - Loaded, managed, and swapped as needed within the available hardware capacity.
  - Configurable (e.g. user can opt into a “bigger but slower” local model or a “smaller but faster” one).
