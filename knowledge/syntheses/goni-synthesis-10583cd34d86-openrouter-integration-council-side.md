---
id: GONI-SYNTHESIS-10583CD34D86
title: OpenRouter integration (council side)
type: synthesis
status: draft
implementation_state: specified_only
proposition: Use model="openrouter/auto" for cheap+good, or pin concrete IDs in config.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/remote-llm-architecture.md
  heading: OpenRouter integration (council side)
  revision: 4fc11a4a1fff204c88ed6df6a2bacd84bc6453ce
---

# OpenRouter integration (council side)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
