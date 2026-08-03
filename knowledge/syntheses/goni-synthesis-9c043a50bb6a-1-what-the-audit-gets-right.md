---
id: GONI-SYNTHESIS-9C043A50BB6A
title: 1. What the audit gets right
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni must be defined by governance, not by a bundle of tools such as LangGraph, MCP, LiteLLM, Ollama, OpenClaw, Qdrant, Home Assistant, or any future substrate.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/55-sovereign-operator-audit-gap-map.md
  heading: 1. What the audit gets right
  revision: 42acf7b164bf9f71154d2bf6c242e753fc43b714
---

# 1. What the audit gets right

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. What the audit gets right

- Goni must be defined by governance, not by a bundle of tools such as
  LangGraph, MCP, LiteLLM, Ollama, OpenClaw, Qdrant, Home Assistant, or any
  future substrate.
- Governance exists to make delegation safe enough that the user can interact
  less, not to make the stack elegant for its own sake.
- Retrieval must become governed **Knowledge & Context Engineering**, not demo
  RAG (`chunk -> embed -> top-k -> generate`).
- Model routing is a policy decision over privacy, evidence quality, latency,
  energy, assurance, and fallback chains; it is not only cost optimization.
- Tool protocols such as MCP are useful, but every effectful call still needs
  Goni-issued authority, sandbox classification, approval/corridor evaluation,
  and receipts.
- Sandboxing is part of the trust boundary. Isolation level follows action
  risk, reversibility, and external side effects.
- Observability is insufficient without evaluation gates that answer whether a
  decision was allowed, reversible, properly routed, and supported by permitted
  memory.
- Browser and online-service automation are not "chat with tools." They are a
  permissioned interface to the web: inspect, compare, draft, fill only with
  approval, track expected responses, keep evidence, and receipt outcomes.
