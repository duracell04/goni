---
id: AGENT-ONTOLOGY-01
title: Agent system ontology
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni treats models, agents, harnesses, retrieval, memory, tools, and agentic systems as distinct composable system roles rather than successive maturity stages.
domains:
- system
- research
- agents
aliases:
- agent ontology
- agentic system ontology
relations:
- type: refines
  target: GONI-THESIS-E1FB8B4F7772
- type: synthesizes
  target: GONI-SYNTHESIS-2F73035842DF
sources:
- SRC-YAO2023-REACT
- SRC-SCHICK2023-TOOLFORMER
- SRC-PLAAT2025-AGENTIC-LLM-SURVEY
artifacts: []
uncertainty: Agent and agentic-AI terminology is not fully standardized across the literature; these definitions are Goni's canonical conceptual vocabulary, informed by the cited literature.
legacy: []
---

# Agent system ontology

Goni rejects the common ladder:

```text
LLM -> RAG -> Agent -> Agentic AI
```

as a literal architecture or maturity sequence. Those terms refer to different system roles and capabilities that can be composed in multiple ways.

## Canonical distinctions

**Model.** A learned component that maps inputs to outputs. A language model can reason, classify, generate, select actions, or propose tool calls, but model inference alone does not constitute a complete agent.

**Harness or agent runtime.** The software substrate around one or more models that constructs context, exposes tool schemas, parses model outputs, dispatches tools, manages state, enforces budgets and permissions, records observations, and determines when the loop stops or continues.

**Agent.** A bounded process that pursues an objective over time by maintaining task state, selecting actions, receiving observations from an environment, and iterating under a runtime. The policy used to select actions may contain an LLM, deterministic code, specialized models, or combinations of these.

**Agentic system.** A larger system that delegates meaningful sequential decision-making across time. Agenticness is therefore a property of the control relationship and execution loop, not a synonym for multi-agent architecture.

**RAG.** Retrieval-augmented generation is an information-acquisition and context-construction mechanism. An agent may use RAG, invoke retrieval as a tool, receive automatically retrieved context, or operate without retrieval.

**Multi-agent system.** A system containing multiple independently stateful agent processes. Multiple agents are an architectural choice, not a higher maturity level than a single well-governed agent.

## Goni interpretation

The core Goni loop remains:

```text
objective
  -> context and state
  -> model-supported decision
  -> proposed action
  -> kernel authorization
  -> tool execution
  -> observation
  -> verification
  -> state update
  -> continue / replan / stop
```

ReAct is a historical reference for interleaving reasoning and acting, while Toolformer is a reference for learned tool selection. Goni adds a stronger systems boundary: a model may propose an action, but the runtime and kernel determine whether and how that proposal becomes an effect.

This ontology preserves the existing doctrine:

> Models reason. The kernel authorizes. Tools act. Receipts prove.
