---
id: AGENT-SYSTEMS-MAP-01
title: Agent systems academic map
type: synthesis
status: draft
implementation_state: specified_only
proposition: The agent-systems research layer connects Goni's authority architecture to explicit models of agent ontology, partial observability, context and memory, interfaces, safe tool use, auditability, trajectory reliability, multi-agent evaluation, consequence simulation, and governed adaptation.
domains:
- research
- system
- agents
aliases:
- agent systems research map
relations:
- type: refines
  target: GONI-SYNTHESIS-F0626F7EFE2C
- type: synthesizes
  target: AGENT-ONTOLOGY-01
- type: synthesizes
  target: AGENT-STATE-01
- type: synthesizes
  target: RAG-CONTEXT-01
- type: synthesizes
  target: TOOL-EFFECT-01
- type: synthesizes
  target: AGENTICNESS-01
- type: synthesizes
  target: INTERFACE-MODEL-01
- type: synthesizes
  target: INTEROP-MCP-A2A-01
- type: synthesizes
  target: INTERFACE-SELECTION-01
- type: synthesizes
  target: TRUST-INPUT-01
- type: synthesizes
  target: AUDIT-RUNTIME-01
- type: synthesizes
  target: TRAJECTORY-EVAL-01
- type: synthesizes
  target: MULTIAGENT-EVAL-01
- type: synthesizes
  target: WORLD-MODEL-01
- type: synthesizes
  target: ADAPTATION-LEVELS-01
sources:
- SRC-YAO2023-REACT
- SRC-SCHICK2023-TOOLFORMER
- SRC-PLAAT2025-AGENTIC-LLM-SURVEY
- SRC-ZHANG2026-AGENTIC-RL
- SRC-MEI2025-CONTEXT-ENGINEERING
- SRC-HU2026-AGENT-MEMORY-SURVEY
- SRC-MCP-2026-07-28
- SRC-A2A-1-0
- SRC-DOSHI2026-SAFE-TOOL-USE
- SRC-WALLACE2024-INSTRUCTION-HIERARCHY
- SRC-DEBENEDETTI2024-AGENTDOJO
- SRC-NIAN2026-AUDITABLE-AGENTS
- SRC-LU2024-TOOLSANDBOX
- SRC-YAO2024-TAUBENCH
- SRC-JIMENEZ2023-SWEBENCH
- SRC-YUAN2026-OSWORLD2
- SRC-TRAN2026-COMPUTE-NORMALIZED-MULTIAGENT
- SRC-YANG2026-WORLD-MODELS
- SRC-GAO2026-SELF-EVOLVING-AGENTS
- SRC-CHEN2026-CONTINUAL-EXPERIENCE
artifacts: []
uncertainty: This map integrates current research into Goni's draft architecture; it does not promote any specified-only mechanism to implemented or verified status.
legacy: []
---

# Agent systems academic map

This node is the navigation layer for the academic agent-system additions.

## Core ontology

- **AGENT-ONTOLOGY-01** separates model, agent, harness/runtime, RAG, and multi-agent architecture.
- **AGENTICNESS-01** defines agenticness as delegated sequential authority.
- **AGENT-STATE-01** adds partial observability, observations, belief state, task state, memory, and current context.
- **RAG-CONTEXT-01** places retrieval inside broader context engineering.
- **TOOL-EFFECT-01** separates epistemic access from causal capability.

## Interfaces and interoperability

- **INTERFACE-MODEL-01** separates human interaction, software invocation, triggers, semantic interoperability, transport, and action surfaces.
- **INTEROP-MCP-A2A-01** distinguishes tool/context interoperability from independent agent interoperability.
- **INTERFACE-SELECTION-01** selects interfaces by structure and observability instead of an absolute API/CLI/GUI ranking.

## Safety and accountability

- **TRUST-INPUT-01** preserves instruction and data trust boundaries.
- **AUDIT-RUNTIME-01** maps Goni receipts onto explicit auditability dimensions.

## Evaluation and research frontiers

- **TRAJECTORY-EVAL-01** evaluates long-horizon, repeated, externally verified delegated behavior.
- **MULTIAGENT-EVAL-01** requires compute-normalized single-agent baselines.
- **WORLD-MODEL-01** treats consequence simulation as optional advisory cognition.
- **ADAPTATION-LEVELS-01** governs persistent adaptation by target, reversibility, evidence, and blast radius.

The intended research target is **reliable delegated autonomy**: increasing useful system initiative only where state, authority, evidence, verification, and recovery remain governable.
