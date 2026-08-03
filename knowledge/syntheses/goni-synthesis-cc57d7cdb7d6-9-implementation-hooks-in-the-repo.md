---
id: GONI-SYNTHESIS-CC57D7CDB7D6
title: 9. Implementation hooks in the repo
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Orchestrator interface: see blueprint/software/30-components/orchestrator.md and router policy in blueprint/software/20-architecture.md for when escalation is allowed.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 9. Implementation hooks in the repo
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 9. Implementation hooks in the repo

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. Implementation hooks in the repo
- Orchestrator interface: see `blueprint/software/30-components/orchestrator.md` and router policy in `blueprint/software/20-architecture.md` for when escalation is allowed.
- Config: `goni-prototype-lab:config/council.yaml` (or env vars) is the ground truth for seats/weights.
- Inspiration: council pattern (multi-answer, peer review, synth); no hard dependency on external repos.
- Remote path details: see `blueprint/docs/remote-llm-architecture.md` for the mediated Council -> OpenRouter data path, budgets, and runtime modes.
- AI-2027 corpus: optional local snapshot lives under `blueprint/docs/assets/ai-2027` for safety/strategy RAG; include that path when ingesting docs.
