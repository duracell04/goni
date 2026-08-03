---
id: GONI-SPEC-AB677E5EC193
title: 7.2 Everyday Interaction
type: specification
status: draft
implementation_state: specified_only
proposition: 'The primary interaction channels should include: Web UI (chat, configuration, dashboards), API for integrations, Optional mobile and desktop clients.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 7.2 Everyday Interaction
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 7.2 Everyday Interaction

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 7.2 Everyday Interaction

- The primary interaction channels should include:
  - Web UI (chat, configuration, dashboards),
  - API for integrations,
  - Optional mobile and desktop clients.

- Users should be able to:
  - see what the system is currently doing (e.g. indexing, training, idle),
  - view basic resource usage (e.g. "node is under heavy load, expect slower replies"),
  - manage connected data sources (add/remove email accounts, storage locations, etc.).
  - control **user-in-the-loop gates** for irreversible actions (send/move/transfer) and choose Socratic vs auto modes per surface.
