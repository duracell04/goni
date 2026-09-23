---
id: EVID-TAUBENCH-RELIABILITY-01
title: 'Source claim: tau-bench measures external state and repeated-run reliability'
type: evidence
status: draft
implementation_state: not_applicable
proposition: tau-bench evaluates agent success by comparing final database state with an annotated goal state and introduces pass^k to measure whether behavior remains successful across repeated trials.
domains:
- research
- evaluation
- reliability
aliases: []
relations:
- type: supports
  target: TRAJECTORY-EVAL-01
sources:
- SRC-YAO2024-TAUBENCH
artifacts: []
uncertainty: pass^k is one reliability metric and should complement, not replace, workflow-specific safety and quality measures.
legacy: []
---

# Source claim: tau-bench measures external state and repeated-run reliability

tau-bench demonstrates two evaluation principles directly relevant to Goni:

- determine success from the resulting environment state rather than asking the agent whether it succeeded;
- repeat tasks to measure consistency, because one successful trajectory does not establish reliable delegated behavior.
