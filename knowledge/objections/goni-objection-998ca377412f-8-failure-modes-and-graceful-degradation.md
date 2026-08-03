---
id: GONI-OBJECTION-998CA377412F
title: 8. Failure modes and graceful degradation
type: objection
status: draft
implementation_state: not_applicable
proposition: 'One model fails (timeout/rate limit): proceed with partial council.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 8. Failure modes and graceful degradation
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 8. Failure modes and graceful degradation

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Failure modes and graceful degradation
- One model fails (timeout/rate limit): proceed with partial council.
- All cloud calls fail: fall back to best available single-model answer or local-only and surface the limitation.
- Chairman fails: pick best-scored member answer or reroute to an alternate chair.
- Never block user flow on council failure.
