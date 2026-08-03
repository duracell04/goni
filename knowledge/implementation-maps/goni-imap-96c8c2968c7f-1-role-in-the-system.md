---
id: GONI-IMAP-96C8C2968C7F
title: 1. Role in the system
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The **LLM Runtime** is the Execution Plane (??) component that: Executes model inference for a given PromptPlan and immutable model bundle, Abstracts over concrete model backends (llama.cpp, vLLM, etc.), Exposes capabilities, utilisation, and active bundle metadata back to the Control Plane (??).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/llm-runtime.md
  heading: 1. Role in the system
  revision: 6ce37ef5d3a676fd26377a3fa8a15c5b226016c2
---

# 1. Role in the system

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Role in the system

The **LLM Runtime** is the Execution Plane (??) component that:

- Executes model inference for a given PromptPlan and immutable model bundle,
- Abstracts over concrete model backends (llama.cpp, vLLM, etc.),
- Exposes capabilities, utilisation, and active bundle metadata back to the
  Control Plane (??).

It is the only component allowed to “speak” to GPUs/NPUs and LLM backends directly.

---
