---
id: GONI-SPEC-B9D59318C711
title: 1. Definition
type: specification
status: draft
implementation_state: specified_only
proposition: 'A Goni OS Agent is a **local userland process** that: reads kernel-maintained latent state, invokes capability-scoped system calls (tools), and requests solver/LLM compute only when escalation is justified.'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agent-definition.md
  heading: 1. Definition
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 1. Definition

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Definition

A Goni OS Agent is a **local userland process** that:

- reads kernel-maintained latent state,
- invokes capability-scoped system calls (tools),
- and requests solver/LLM compute only when escalation is justified.

Agents do not own global state and have no ambient authority.
