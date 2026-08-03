---
id: GONI-IMAP-CE9254BA360D
title: 3. Router Regret Analysis (Control→Execution)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Inputs: 𝒦.RouterDecisions (policy choice + features), ℰ.LlmCalls (latency/tokens), ℰ.Metrics (quality proxies).'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/52-zero-copy-mechanics.md
  heading: 3. Router Regret Analysis (Control→Execution)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 3. Router Regret Analysis (Control→Execution)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Router Regret Analysis (Control→Execution)
- Inputs: 𝒦.RouterDecisions (policy choice + features), ℰ.LlmCalls (latency/tokens), ℰ.Metrics (quality proxies).
- Process: join on `request_id`; compute regret = f(features, latency, quality); populate `offline_reward_estimate`.
- Copies: 0; Arrow compute over numeric/dictionary columns.
