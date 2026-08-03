---
id: GONI-EXPERIMENT-46963ADA140C
title: Scenario
type: experiment
status: draft
implementation_state: not_applicable
proposition: Fix corpus, task, and latency/token/tool budgets.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-LONGCTX-02-corpus-reading-fidelity.md
  heading: Scenario
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Scenario

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Scenario
- Fix corpus, task, and latency/token/tool budgets.
- Compare answer claims against gold evidence spans.
- Evaluate whether corpus-reading loses or distorts evidence that RAG/context
  assembly would preserve.
