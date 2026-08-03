---
id: GONI-SYNTHESIS-6CED3E236633
title: Runtime modes (AI-2027 informed)
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Normal:** Remote allowed for high-value tasks; use configured seats/auto-router; soft budgets and logging on.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/remote-llm-architecture.md
  heading: Runtime modes (AI-2027 informed)
  revision: 4fc11a4a1fff204c88ed6df6a2bacd84bc6453ce
---

# Runtime modes (AI-2027 informed)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Runtime modes (AI-2027 informed)
- **Normal:** Remote allowed for high-value tasks; use configured seats/auto-router; soft budgets and logging on.
- **Constrained / AGI-risk:** Strong daily cap; whitelist remote tools; prefer deterministic/local for automation; restrict models to cheap/strict list.
- **Offline / AI blackout:** Cloud path disabled; router surfaces limitation; stay local for summarisation, Arrow search, scheduling, embeddings.
- Mode can be a goni-prototype-lab:config switch (e.g., `REMOTE_MODE=normal|constrained|offline`).
