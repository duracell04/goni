---
id: GONI-SYNTHESIS-1AE3502966E2
title: 10. Example config sketch (informative, not binding)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 10. Example config sketch (informative, not binding)
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 10. Example config sketch (informative, not binding)
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 10. Example config sketch (informative, not binding)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
