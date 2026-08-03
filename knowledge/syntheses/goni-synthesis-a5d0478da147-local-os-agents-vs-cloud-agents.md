---
id: GONI-SYNTHESIS-A5D0478DA147
title: Local OS agents vs cloud agents
type: synthesis
status: draft
implementation_state: specified_only
proposition: Cloud agents are typically remote workflows that loop over LLM tokens and external tools.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-whitepaper.md
  heading: Local OS agents vs cloud agents
  revision: 66b954ceb474004d6304fd1fb280804bae3e7e6b
---

# Local OS agents vs cloud agents

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Local OS agents vs cloud agents

Cloud agents are typically remote workflows that loop over LLM tokens and
external tools. Goni OS agents are **local processes** running under a kernel
that owns state, policy, and budgets. The LLM is a **rare, budgeted interrupt**
used for ambiguity resolution or human-facing output, not the control loop.

This is infrastructure-first and model-agnostic: JEPA-style latent prediction is
compatible inspiration, not a mandatory training objective.
