---
id: GONI-SYNTHESIS-21511DAEF5B3
title: 10. Inputs from Goni Lab (LM Arena–style evals)
type: synthesis
status: draft
implementation_state: specified_only
proposition: The **Goni Lab** mode (blueprint/docs/goni-lab.md) runs LM Arena–style comparisons across local seats (vLLM/TGI/Ollama/LM Studio) and council seats (OpenRouter/provider IDs).
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 10. Inputs from Goni Lab (LM Arena–style evals)
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 10. Inputs from Goni Lab (LM Arena–style evals)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 10. Inputs from Goni Lab (LM Arena–style evals)
- The **Goni Lab** mode (`blueprint/docs/goni-lab.md`) runs LM Arena–style comparisons across local seats (vLLM/TGI/Ollama/LM Studio) and council seats (OpenRouter/provider IDs).
- Lab logs latency, tokens, refusal/safety, faithfulness (verifier), and quick win/lose/tie ratings per task tag. These become supervised data for `goni-router` regret tests and for proposing seat/routing changes.
- Council seats should be adjusted only with evidence from Lab runs (champion labels per task tag, cost/latency deltas). Lab “promote” writes a proposed patch to `goni-prototype-lab:config/council.yaml`; ops review/merge.
- Web-grounded seats (Perplexity Sonar, Grok) are opt-in and tagged; they are used only when the task tag demands live/current information.
- The Frugal Sovereign Routing evidence lane measures false local accepts, late
  escalation, wasted cloud calls, privacy-risk overrides, and whether Council
  disagreement checking improved the final local synthesis.
