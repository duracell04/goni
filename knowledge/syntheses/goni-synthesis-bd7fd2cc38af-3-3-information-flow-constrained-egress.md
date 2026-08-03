---
id: GONI-SYNTHESIS-BD7FD2CC38AF
title: 3.3 Information-flow constrained egress
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Default runtime posture is confinement: tool sandboxes run without ambient egress, policy-governed declassification is required for outbound flow, data classes drive redaction and release decisions.'
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
  heading: 3.3 Information-flow constrained egress
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 3.3 Information-flow constrained egress

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Information-flow constrained egress

Default runtime posture is confinement:
- tool sandboxes run without ambient egress,
- policy-governed declassification is required for outbound flow,
- data classes drive redaction and release decisions.

Related foundations:
- OS-level IFC with minimal trusted core [[zeldovich2006-histar]]
- IFC integrated with standard OS abstractions [[krohn2007-flume]].
