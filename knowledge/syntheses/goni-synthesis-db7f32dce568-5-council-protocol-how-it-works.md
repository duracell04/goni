---
id: GONI-SYNTHESIS-DB7F32DCE568
title: 5. Council protocol (how it works)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Stage 1 - Collection: orchestrator fans out the structured/trimmed prompt to all members; each returns an independent answer.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 5. Council protocol (how it works)
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 5. Council protocol (how it works)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Council protocol (how it works)
- Stage 1 - Collection: orchestrator fans out the structured/trimmed prompt to all members; each returns an independent answer.
- Stage 2 - Peer review (anonymized): each model sees all answers without brand IDs; scores/critiques them.
- Stage 3 - Synthesis: chairman gets question + answers + critiques/scores; emits final answer plus optional confidence/diversity note.
- Integration: orchestrator treats `council` as another backend with higher latency/cost but (expected) higher reliability.
