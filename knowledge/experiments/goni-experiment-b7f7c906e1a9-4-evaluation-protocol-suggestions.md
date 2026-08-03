---
id: GONI-EXPERIMENT-B7F7C906E1A9
title: 4. Evaluation protocol suggestions
type: experiment
status: draft
implementation_state: not_applicable
proposition: 'The following test classes operationalize the thesis: Mediation completeness tests prove no side effect can occur without policy path invocation.'
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
  heading: 4. Evaluation protocol suggestions
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 4. Evaluation protocol suggestions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Evaluation protocol suggestions

The following test classes operationalize the thesis:

1. Mediation completeness tests
- prove no side effect can occur without policy path invocation.

2. Bypass resistance tests
- deny direct socket/file escape attempts from tool runtime.

3. Capability attenuation tests
- verify delegated authority cannot exceed parent grant.

4. Provenance completeness tests
- verify every mediated action has a receipt with linked decision metadata.

5. Egress and IFC tests
- verify declassification/egress policy behavior by data class.

6. Scheduler stress tests
- mixed interactive/background runs with KV pressure and tail-latency checks.

7. TCB minimization tracking
- keep a versioned map of trusted modules and review every expansion.
