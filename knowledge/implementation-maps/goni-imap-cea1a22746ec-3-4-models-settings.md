---
id: GONI-IMAP-CEA1A22746EC
title: 3.4 Models & Settings
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Shows: List of available models (goni-small, goni-large, …).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 3.4 Models & Settings
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 3.4 Models & Settings

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4 Models & Settings

Shows:

- List of available models (goni-small, goni-large, …).
- Per-model capabilities (max context length, approximate throughput, memory footprint).
- Which backend is used (llama.cpp, vLLM, etc.).
- High-level toggles, e.g.:
  - “local-only mode” (no network use),
  - “RAG enabled by default”,
- “default model tier for interactive jobs”.

Settings changes must map to documented configuration mechanisms (config files or dedicated APIs).
