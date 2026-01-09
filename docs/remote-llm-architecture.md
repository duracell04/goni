# Remote LLM Architecture (Council -> OpenRouter)

> Single cloud leg for Goni: the OS talks to the council; the council talks to OpenRouter; OpenRouter fans out to providers.

## Role in the stack
- Path: Goni OS Task Router -> local backend (vLLM/sglang on-box) or remote backend (Goni Council service).
- Remote backend: Council -> OpenRouter API -> providers/models (OpenAI, Anthropic, DeepSeek, Gemini, etc.) -> optional web/search tools running beside the council.
- External egress is mediated by the Network Gate (NET-01); the Gate selects DIRECT vs OVERLAY routes for Council calls.
- Goni OS never calls provider APIs directly; OpenRouter is the only cloud gateway.
- Council enforces routing policies, budgets, and approval before sending anything out.

## Data path, privacy, and cost controls
- Trim/sanitise context before sending; prefer summaries or extracted facts instead of raw artifacts.
- Enforce per-call and per-day budgets (tokens or $) and drop to local-only when exceeded or unavailable.
- Log every remote call (Arrow table) with model/provider, usage/cost, latency, and decision outcome; keep request/response snippets minimal for privacy.

## OpenRouter integration (council side)
```python
import os
import requests

OPENROUTER_KEY = os.environ["OPENROUTER_API_KEY"]
OPENROUTER_URL = os.environ.get("OPENROUTER_URL", "https://openrouter.ai/api/v1/chat/completions")


def call_openrouter(model: str, messages: list[dict], max_tokens: int = 800):
    headers = {
        "Authorization": f"Bearer {OPENROUTER_KEY}",
        "HTTP-Referer": os.environ.get("OPENROUTER_HTTP_REFERER", "https://goni.local"),
        "X-Title": os.environ.get("OPENROUTER_X_TITLE", "GoniCouncil"),
    }
    body = {
        "model": model,
        "messages": messages,
        "max_tokens": max_tokens,
    }
    resp = requests.post(OPENROUTER_URL, json=body, headers=headers, timeout=60)
    resp.raise_for_status()
    data = resp.json()
    return data["choices"][0]["message"]["content"], data.get("usage", {})
```
- Use `model="openrouter/auto"` for cheap+good, or pin concrete IDs in config.
- Optionally set `HTTP-Referer` and `X-Title` for analytics; keep keys in env/secret store.

## Runtime modes (AI-2027 informed)
- **Normal:** Remote allowed for high-value tasks; use configured seats/auto-router; soft budgets and logging on.
- **Constrained / AGI-risk:** Strong daily cap; whitelist remote tools; prefer deterministic/local for automation; restrict models to cheap/strict list.
- **Offline / AI blackout:** Cloud path disabled; router surfaces limitation; stay local for summarisation, Arrow search, scheduling, embeddings.
- Mode can be a config/env switch (e.g., `REMOTE_MODE=normal|constrained|offline`).

## Where remote compute can live
- Primary cloud leg is Council -> OpenRouter (this doc).
- Optional DePIN/offload targets (Cocoon, Akash, Render, io.net, Spheron, Bittensor, OORT) are catalogued in `hardware/20-architecture-options.md` §7.3; they are potential overflow/backhaul endpoints, not part of the default path.

## AI-2027 as knowledge source
- Keep a snapshot of AI-2027 (PDF/HTML) under `docs/assets/ai-2027/` with a `manifest.json` (see `manifest.json.example`) capturing source URL, retrieved_at, format, file name, and sha256 for deterministic ingestion.
- Index it into the Arrow spine for RAG so safety/strategy answers cite this corpus instead of generic web takes.
- It also motivates constrained/offline modes and multi-provider indirection.

## Config and ops hooks
- Seats/weights and triggers: `config/council.yaml` (ground truth).
- Env defaults for the council service: see `config/council.env.example` (`OPENROUTER_API_KEY`, `OPENROUTER_MODEL`, `OPENROUTER_URL`, `MAX_REMOTE_TOKENS_PER_DAY`, `ALLOW_REMOTE_TOOLS`, `REMOTE_MODE`, optional `OPENROUTER_HTTP_REFERER`/`OPENROUTER_X_TITLE`).
- Web/search tools live on the council side; Goni OS never ships a cloud browser tool in the base box.
