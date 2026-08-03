---
id: GONI-SYNTHESIS-DD4B9AA8B57E
title: 3. Minimal Goni Lab flow
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Inputs**: prompt, Arrow-spine context selector, task tag (coding, planning, search+rag, summarization, creative, etc.), optional deterministic profile.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-lab.md
  heading: 3. Minimal Goni Lab flow
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 3. Minimal Goni Lab flow

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Minimal Goni Lab flow
1) **Inputs**: prompt, Arrow-spine context selector, task tag (`coding`, `planning`, `search+rag`, `summarization`, `creative`, etc.), optional deterministic profile.  
2) **Fan-out** to seats: local (vLLM/TGI, Ollama/LM Studio) + remote (council seats via OpenRouter/per-provider).  
3) **Collect** per-seat stats: latency, input/output tokens, cost (if cloud), refusal/safety flag, tool success rate, retrieval coverage.  
4) **Rate**: quick win/lose/tie or 1–5; optional verifier model judges hallucination/citation.  
5) **Write** an Arrow/Parquet log row: prompt hash, task tag, model id, metrics, rating, verifier signal.  
6) **Surface** a filtered leaderboard by task tag; “promote” action writes a proposed routing rule (e.g., `coding -> codestral, deepseek; fallback llama4`). Ops can review/merge into `goni-prototype-lab:config/council.yaml`.
