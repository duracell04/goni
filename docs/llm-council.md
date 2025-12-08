# Goni LLM Council

> This document defines the Goni LLM Council: a cloud-side, multi-model escalation path used only for hard tasks or long-context queries, and never for default traffic.

## 0. Position in the repo
- Linked from: 2.1 Use-case constraints, `software/20-architecture.md` (router notes), and `docs/README.md`.
- Config ground truth: `config/council.yaml` (or env vars), not this doc.

## 1. What is the LLM Council in Goni?
- A multi-model panel of cloud LLMs (GPT, Claude, Gemini, Grok, etc.) behind the orchestrator, not directly user-facing.
- Follows a council pattern: multiple answers -> peer critique -> a chair synthesizes.
- Optional extension: Goni must remain useful offline and local-first without it.

## 2. Goals and non-goals
- Goals:
  - Raise reliability/robustness for high-stakes reasoning, long context, and explicit "cloud best-effort" runs.
  - Diversify providers to avoid single-vendor lock-in.
  - Enable experiments with voting/ranking/specialists without changing front-end APIs.
- Non-goals:
  - Not the default path for all traffic.
  - Not a pricing abstraction or SaaS broker.
  - Not guaranteed to beat the best single cloud call every time.

## 3. When Goni uses the Council
- Explicit user request: UI/API flag `provider="council"` or `mode="paranoid"` routes to council.
- Orchestrator heuristics (examples):
  - Task classified "high difficulty" or "safety-critical".
  - Context length beyond local model comfort.
  - Sensitive domains where a second opinion is required (e.g., medical/financial summaries).
- Hard constraint: if council is unavailable (no network/keys), orchestrator degrades gracefully ("I can only use local models right now") rather than blocking or crashing.

## 4. Council composition (families and roles)
- Roles:
  - Members: generate first-pass answers and reviews.
  - Chairman: synthesizes final answer; can be distinct or one of the members.
  - Specialists: code, web-grounded, reasoning, safety.
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
- Declaration: single config (`config/council.yaml`) or env vars (`COUNCIL_MODELS`, `CHAIRMAN_MODEL`). Repo ships a reference council; deployments may override.

## 5. Council protocol (how it works)
- Stage 1 - Collection: orchestrator fans out the structured/trimmed prompt to all members; each returns an independent answer.
- Stage 2 - Peer review (anonymized): each model sees all answers without brand IDs; scores/critiques them.
- Stage 3 - Synthesis: chairman gets question + answers + critiques/scores; emits final answer plus optional confidence/diversity note.
- Integration: orchestrator treats `council` as another backend with higher latency/cost but (expected) higher reliability.

## 6. Privacy, data routing, and keys
- Data path: only the prompt/context needed for the task goes to cloud; keep local artifacts local where possible.
- Redaction: orchestrator should strip direct identifiers or use summaries when feasible before cloud send.
- Keys: stored locally (encrypted config); council use is opt-in and requires explicit configuration.

## 7. Cost and performance model
- Assumptions: council is slower and more expensive than local or single-cloud.
- Guards: soft budget per query (max tokens across members), daily/monthly soft limits.
- Tracking: log tokens per council call, cost per model, added latency.

## 8. Failure modes and graceful degradation
- One model fails (timeout/rate limit): proceed with partial council.
- All cloud calls fail: fall back to best available single-model answer or local-only and surface the limitation.
- Chairman fails: pick best-scored member answer or reroute to an alternate chair.
- Never block user flow on council failure.

## 9. Implementation hooks in the repo
- Orchestrator interface: see `software/30-components/orchestrator.md` and router policy in `software/20-architecture.md` for when escalation is allowed.
- Config: `config/council.yaml` (or env vars) is the ground truth for seats/weights.
- Inspiration: council pattern (multi-answer, peer review, synth); no hard dependency on external repos.
- Remote path details: see `docs/remote-llm-architecture.md` for the mediated Council -> OpenRouter data path, budgets, and runtime modes.
- AI-2027 corpus: optional local snapshot lives under `docs/assets/ai-2027` for safety/strategy RAG; include that path when ingesting docs.

## 10. Example config sketch (informative, not binding)
```yaml
chairman: openai:gpt-4.1
members:
  - openai:gpt-4.1-mini
  - anthropic:claude-sonnet-4
  - google:gemini-2.5-pro
  - deepseek:v3.2-speciale
  - mistral:large-3
  - meta:llama-4-maverick
specialists:
  web: perplexity:sonar-pro
  news: xai:grok-4
  code: mistral:codestral-mamba
  reasoning: deepseek:r1
budget:
  max_tokens_total: 200_000
  max_models: 6
timeouts:
  per_call_ms: 30000
  total_ms: 90000
fallback:
  on_chair_fail: best_scored_member
  on_all_fail: local_only
```
