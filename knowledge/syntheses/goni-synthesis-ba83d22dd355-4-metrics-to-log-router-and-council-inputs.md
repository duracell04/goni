---
id: GONI-SYNTHESIS-BA83D22DD355
title: 4. Metrics to log (router and council inputs)
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Latency** (p50/p95) and **tokens** (in/out) -> capacity planning and MaxWeight service rates.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-lab.md
  heading: 4. Metrics to log (router and council inputs)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 4. Metrics to log (router and council inputs)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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
