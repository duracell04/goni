---
id: GONI-EXPERIMENT-BEC00C3480C2
title: F) Observability, evaluation, and model gateway/router layer
type: experiment
status: draft
implementation_state: not_applicable
proposition: '| Project | Confidence | Goni relevance | | LiteLLM | verified | Model-agnostic proxy/router across local and cloud providers.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: F) Observability, evaluation, and model gateway/router layer
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# F) Observability, evaluation, and model gateway/router layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### F) Observability, evaluation, and model gateway/router layer

| Project | Confidence | Goni relevance |
| --- | --- | --- |
| LiteLLM | `verified` | Model-agnostic proxy/router across local and cloud providers. |
| Portkey | `verified` | AI gateway, routing, guardrail, and observability candidate. |
| Helicone | `verified` | LLM observability and gateway candidate. |
| Langfuse | `verified` | Open-source LLM tracing, prompt, eval, and cost observability. |
| AgentOps | `verified` | Agent-specific observability candidate. |
| LangSmith | `verified` | LangChain ecosystem tracing/evals platform. |
| Arize Phoenix | `verified` | Open-source observability/evaluation stack. |
| Bench360 | `needs verification` | Academic benchmark candidate for local inference comparisons. |
| PASB | `candidate/unverified` | Personalized Agent Security Benchmark claim needs primary source check. |
| OpenClaw-RL | `candidate/unverified` | RL personalization extension claim needs primary source check. |

Goni implication:

- Receipts are not the same as observability traces; Goni should support both.
- Routers like LiteLLM are useful plumbing, but the policy decision remains in
  Goni's control plane.
