---
id: GONI-SYNTHESIS-62CDE6901FA0
title: 3.4 Provenance-by-construction ("receipts")
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Effectful actions should emit mandatory structured provenance: entities, activities, and agents for each mediated step, causal linkage for inputs, decisions, and outputs, trace propagation for cross-component correlation.'
domains:
- agent
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/40-agentic-kernel-foundations.md
  heading: 3.4 Provenance-by-construction ("receipts")
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 3.4 Provenance-by-construction ("receipts")

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4 Provenance-by-construction ("receipts")

Effectful actions should emit mandatory structured provenance:
- entities, activities, and agents for each mediated step,
- causal linkage for inputs, decisions, and outputs,
- trace propagation for cross-component correlation.

Related foundations:
- W3C PROV family [[w3c2013-prov]]
- W3C Trace Context [[w3c2021-trace-context]].
