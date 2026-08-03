---
id: GONI-EXPERIMENT-A37CBB6700ED
title: Scenario
type: experiment
status: draft
implementation_state: not_applicable
proposition: Run corpus-reading and hybrid strategies under fixed latency, token, and tool budgets.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-LONGCTX-03-recursion-budget-safety.md
  heading: Scenario
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Scenario

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Scenario
- Run corpus-reading and hybrid strategies under fixed latency, token, and tool
  budgets.
- Stress recursive decomposition on tasks that tempt over-scanning.
- Record whether the strategy degrades relative to the native in-window
  baseline on shorter inputs.
