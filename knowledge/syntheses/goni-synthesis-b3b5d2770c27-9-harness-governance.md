---
id: GONI-SYNTHESIS-B3B5D2770C27
title: 9) Harness Governance
type: synthesis
status: draft
implementation_state: specified_only
proposition: Approval, edit, override, and rejection rates by task class and harness component.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics.md
  heading: 9) Harness Governance
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# 9) Harness Governance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 9) Harness Governance
- Approval, edit, override, and rejection rates by task class and harness
  component.
- Failed retrieval rate and wrong-routing rate for changes to retrieval and
  model-routing policy.
- Rollback rate, rollback success rate, and policy violation rate after harness
  promotion.
- Prediction hit rate for harness changes: percent of declared predictions that
  hold over their stated eval window without safety, cost, or latency
  regression.
