---
id: GONI-PRINCIPLE-3CE61BB5417A
title: 0.0b Sovereign modularity principle
type: principle
status: draft
implementation_state: specified_only
proposition: Goni must remain modular, sovereign, and independently governable.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 0.0b Sovereign modularity principle
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 0.0b Sovereign modularity principle

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 0.0b Sovereign modularity principle

Goni must remain modular, sovereign, and independently governable.

Goni is a sovereign control plane over replaceable AI substrate. Third-party
tools may provide orchestration, inference, retrieval, user interfaces, voice,
workflow automation, model gateways, and hardware acceleration, but they must
never own the kernel contracts. Goni-owned authority lives in Work Orders, Done
Contracts, receipts, capability tokens, model provenance, memory governance,
autonomy corridors, policy packs, install receipts, rollback references, and
promotion semantics.

The architecture therefore follows five hard rules:

1. **Interfaces before implementations.** Goni defines kernel contracts before
   choosing substrates. LangGraph, LiteLLM, Qdrant, OpenClaw, Open WebUI,
   Ollama, Home Assistant, Hugging Face tools, and similar systems plug into
   Goni-owned contracts rather than defining them.
2. **No substrate owns canonical state.** Third-party tools may execute,
   display, retrieve, route, parse, or speak. They must not become the
   canonical owner of memory, permissions, policy, approvals, receipts,
   identity, or rollback state.
3. **Local authority beats remote convenience.** Cloud services may be council
   members, fallback models, parsers, or research tools, but they never become
   the default trust boundary. Any route that moves data, memory, tools, or
   decisions outside the local system must be policy-approved and receipted.
4. **Portability is a design requirement.** Each layer needs an adapter
   boundary: orchestration, runtime, memory, UI, voice, model gateway, and
   tool execution. If a dependency changes license, degrades, is acquired,
   becomes unsafe, or stops being maintained, Goni must be able to swap it
   without rewriting the kernel.
5. **Promotion into the core is slow.** New tools, models, policies, memory
   strategies, and workflows can move quickly at the edge. They move inward
   only after evidence: install receipt, eval receipt, policy compatibility,
   security review, sandbox classification, rollback path, and measured
   improvement.

Modularity is not only convenience. It is the mechanism by which Goni remains
independent: every external dependency must be replaceable, and every
governance decision must remain local, inspectable, and receipted.

---
