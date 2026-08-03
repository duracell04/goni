---
id: GONI-IMAP-DC491388E35A
title: 2.2 Non-responsibilities
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '? Choosing which model tier to use (router). ? Selecting RAG context (context selector). ? Managing global queueing or admission control. ? Deciding whether speculative drafts are accepted, shortened, escalated, or sent to a council. Those decisions stay in the router/control plane. ? Mutating base weights, promoting'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/llm-runtime.md
  heading: 2.2 Non-responsibilities
  revision: 6ce37ef5d3a676fd26377a3fa8a15c5b226016c2
---

# 2.2 Non-responsibilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Non-responsibilities

- ? Choosing which model tier to use (router).  
- ? Selecting RAG context (context selector).  
- ? Managing global queueing or admission control.
- ? Deciding whether speculative drafts are accepted, shortened, escalated, or
  sent to a council. Those decisions stay in the router/control plane.
- ? Mutating base weights, promoting patches, or learning online in production.

---
