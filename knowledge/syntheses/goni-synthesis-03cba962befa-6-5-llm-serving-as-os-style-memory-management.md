---
id: GONI-SYNTHESIS-03CBA962BEFA
title: 6.5 LLM serving as OS-style memory management
type: synthesis
status: draft
implementation_state: specified_only
proposition: Modern LLM serving explicitly uses OS-like paging for KV cache and couples it to scheduling, providing a precedent for preemption/cancellation as first-class control.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: 6.5 LLM serving as OS-style memory management
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# 6.5 LLM serving as OS-style memory management

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.5 LLM serving as OS-style memory management

Modern LLM serving explicitly uses OS-like paging for KV cache and couples it
to scheduling, providing a precedent for preemption/cancellation as first-class
control. [R15]

Goni mapping (normative):
- LLM runtime exposes utilization/capability signals so the scheduler can
  protect interactive QoS and preempt background inference.
