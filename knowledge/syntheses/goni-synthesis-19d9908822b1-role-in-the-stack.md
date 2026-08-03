---
id: GONI-SYNTHESIS-19D9908822B1
title: Role in the stack
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Path: Goni OS Task Router -> local memory/tools/RAG/models first -> remote backend (Goni Council service) only when local routes are insufficient, too slow, or explicitly requested.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/remote-llm-architecture.md
  heading: Role in the stack
  revision: 4fc11a4a1fff204c88ed6df6a2bacd84bc6453ce
---

# Role in the stack

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Role in the stack
- Path: Goni OS Task Router -> local memory/tools/RAG/models first -> remote
  backend (Goni Council service) only when local routes are insufficient,
  too slow, or explicitly requested.
- Remote backend: Council -> OpenRouter API -> providers/models (OpenAI, Anthropic, DeepSeek, Gemini, etc.) -> optional web/search tools running beside the council.
- External egress is mediated by the Network Gate (NET-01); the Gate selects DIRECT vs OVERLAY routes for Council calls.
- Goni OS never calls provider APIs directly; OpenRouter is the only cloud gateway.
- Council enforces routing policies, budgets, and approval before sending anything out.
- The routing decision is a receipted event with the local attempt, escalation
  reason, redaction state, privacy class sent, and cost/latency estimates.
