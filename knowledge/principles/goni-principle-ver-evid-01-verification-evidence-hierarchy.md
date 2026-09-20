---
id: GONI-PRINCIPLE-VER-EVID-01
title: "Verification Evidence Hierarchy"
type: principle
status: draft
implementation_state: specified_only
proposition: "Goni should prefer verification channels with stronger external grounding and lower dependence on the producing model, while recording what property each channel actually establishes."
domains:
- evaluation
- system
aliases: []
relations:
- type: refines
  target: GONI-SPEC-FD6CC59ECC08
- type: refines
  target: GONI-SPEC-43169414D3A8
sources: []
artifacts: []
uncertainty: "The ordering is a default engineering heuristic rather than a universal total order; task-specific evidence quality can reverse adjacent levels."
legacy: []
---

# Verification Evidence Hierarchy

Goni should seek the strongest task-appropriate evidence available. A default ordering is:

```text
external ground truth
> deterministic executable test
> independent external evidence
> independent model evaluator
> producer self-review
```

The hierarchy measures grounding and independence, not rhetorical confidence. Each verification result should identify the property checked, the evidence source, and the limitations of that check.

Verification evidence supports a receipt; it does not convert an open-ended task into mathematical proof.
