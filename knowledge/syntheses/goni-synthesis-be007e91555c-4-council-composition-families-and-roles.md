---
id: GONI-SYNTHESIS-BE007E91555C
title: 4. Council composition (families and roles)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Roles: Members: generate first-pass answers and reviews.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 4. Council composition (families and roles)
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 4. Council composition (families and roles)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Council composition (families and roles)
- Roles:
  - Members: generate first-pass answers and reviews.
  - Chairman: synthesizes final answer; can be distinct or one of the members.
  - Specialists: code, web-grounded, reasoning, safety.
  - Asymmetric reviewers: prefer assigned functions over generic duplicate
    opinions, e.g. main answer, critic, privacy/risk checker, action-safety
    checker, and style/domain reviewer.
- Families to seat (bind exact IDs in config, not here):
  - OpenAI: GPT-4.1 (chair), GPT-4.1-mini or GPT-4o (fast vote).
  - Anthropic: Claude Opus 4.x / 4.5 (alt chair), Claude Sonnet 4.x, Haiku 4.5.
  - Google: Gemini 2.5 Pro (reasoning), Gemini 2.5 Flash / Flash-Lite (fast multimodal).
  - DeepSeek: V3.2 / V3.2-Speciale (reasoning/code), R1 (reasoner), Coder (code).
  - Mistral: Large 3 (generalist), Medium 3.1, Small 3.2 / Minstral 3 (fast), Codestral / Codestral-Mamba (code).
  - Meta Llama: Llama 3.x 70B/405B instruct, Llama 4 Maverick (generalists).
  - Cohere: Command A (03-2025) for RAG/agents.
  - Perplexity: Sonar Pro / Sonar Reasoning Pro (web-grounded/fact-check).
  - xAI: Grok-4 (live/current events).
- Declaration: single config (`goni-prototype-lab:config/council.yaml`) or env vars (`COUNCIL_MODELS`, `CHAIRMAN_MODEL`). Repo ships a reference council; deployments may override.
