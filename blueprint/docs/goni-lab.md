# Goni Lab (LM Arena–style evaluation layer)

> An internal “multi-model playground” for Goni. Run the same prompt + Arrow context across local and cloud seats, collect metrics, and feed routing policy updates back into the LLM Council.

## 1. Purpose
- Give builders a fast way to **compare models side-by-side** on real Goni tasks (chat, coding, RAG, tools).
- Produce **structured evidence** (latency, cost, refusals, faithfulness) that feeds `goni-router` thresholds and `goni-prototype-lab:config/council.yaml` seating.
- Stay **local-first**: include LM Studio / Ollama seats and offline-only runs; use cloud seats only when allowed.

## 2. Ecosystem references (why this doc exists)
- **LM Arena / LMSYS Chatbot Arena** – side-by-side prompts with Elo/leaderboard crowd signals.
- **LLMArena / Wordware / Airtrain** – "try all the models" UX, latency/throughput breakdowns, dataset-based evals.
- **Perplexity / MultipleChat / Poe** – multi-model chat clients; inspiration for cross-check and citations.
- **Local runners (LM Studio, Jan, Ollama)** – keep a local seat in the grid for privacy/offline.
- We borrow the **comparison + quick rating** loop, not their hosted infra.

## 3. Minimal Goni Lab flow
1) **Inputs**: prompt, Arrow-spine context selector, task tag (`coding`, `planning`, `search+rag`, `summarization`, `creative`, etc.), optional deterministic profile.  
2) **Fan-out** to seats: local (vLLM/TGI, Ollama/LM Studio) + remote (council seats via OpenRouter/per-provider).  
3) **Collect** per-seat stats: latency, input/output tokens, cost (if cloud), refusal/safety flag, tool success rate, retrieval coverage.  
4) **Rate**: quick win/lose/tie or 1–5; optional verifier model judges hallucination/citation.  
5) **Write** an Arrow/Parquet log row: prompt hash, task tag, model id, metrics, rating, verifier signal.  
6) **Surface** a filtered leaderboard by task tag; “promote” action writes a proposed routing rule (e.g., `coding -> codestral, deepseek; fallback llama4`). Ops can review/merge into `goni-prototype-lab:config/council.yaml`.

## 4. Metrics to log (router and council inputs)
- **Latency** (p50/p95) and **tokens** (in/out) -> capacity planning and MaxWeight service rates.
- **Cost** per run and per-1000 tokens (cloud seats) -> budget guardrails.
- **Refusal / safety** rate -> avoid seats that over-refuse or leak.
- **Faithfulness**: verifier score comparing answer to retrieved context; bonus for citations when retrieval is used.
- **Tool use**: success/exception rate per tool chain; was the plan followed?
- **Long-context stability**: degradation beyond N tokens (track N where quality drops).
- **Champion labels**: best-in-class per task tag with timestamp + sample size.
- **ITCR duty cycle**: fraction of time the ITCR reasoner is active vs baseline cognition.
- **Wake overhead**: time-to-first-action after escalation; escalations per hour.
- **Oscillation rate**: repeated escalate/de-escalate cycles (hysteresis effectiveness).
- **Energy-normalized success**: task success per joule or per Wh when power data is available.
- **Action regret**: rollback/undo rate for executed tool actions.

## 5. Integration points in this repo
- **Council config**: keep `goni-prototype-lab:config/council.yaml` as the ground truth; Lab “promote” steps propose edits, not auto-merge.
- **Router training data**: logs become supervised data for `goni-router` regret tests in `blueprint/software/30-conformance.md` (label = chosen/best seat).
- **Data plane**: store logs as Arrow batches (schema lives beside other metrics in `blueprint/software/50-data`); keep prompts hashed with minimal snippets for privacy.
- **Runtime hooks**: Lab is just another mode of the orchestrator UI/CLI; reuse the existing OpenAI-compatible gateways for seat calls.

## 6. Backends to support (priority order)
- **Local**: vLLM/TGI seats; optional Ollama/LM Studio for hobby setups.
- **Cloud (via Council)**: seats already in `goni-prototype-lab:config/council.yaml` (OpenRouter IDs or direct providers).
- **Web-grounded**: Perplexity Sonar / Grok only when task tag demands live info; log separately.
- **Future**: Airtrain-style dataset evals for repeatable benchmarks; slot in when we have labelled task sets.

## 7. Why this matters for Goni
- Tightens the **local-first promise**: we only escalate when Lab data shows a clear win.
- Makes the **Council explainable**: every seat and threshold is backed by on-box measurements, not guesses.
- Reduces **drift**: periodic Lab runs catch regressions when models update upstream.
