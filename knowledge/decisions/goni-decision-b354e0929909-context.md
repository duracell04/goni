---
id: GONI-DECISION-B354E0929909
title: Context
type: decision
status: draft
implementation_state: specified_only
proposition: 'We need an MVP node that is: small (6â€“8 L), quiet, and always-on, capable of running two local OSS models in parallel plus RAG, power-feasible on a standard outlet, and upgradeable by swapping the compute module.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/90-decisions.md
  heading: Context
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Context

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Context

We need an MVP node that is:

- small (6â€“8 L), quiet, and always-on,
- capable of running two local OSS models in parallel plus RAG,
- power-feasible on a standard outlet,
- and upgradeable by swapping the compute module.

`20-architecture-options.md` compares:

1. APU-centric node (CPU+iGPU+NPU + unified LPDDR5X),
2. discrete GPU workstation (x86 + high-end dGPU),
3. external heavy node (Grace/Blackwell class) as add-on.
