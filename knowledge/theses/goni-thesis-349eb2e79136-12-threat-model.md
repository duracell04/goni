---
id: GONI-THESIS-349EB2E79136
title: 12. Threat Model
type: thesis
status: draft
implementation_state: specified_only
proposition: 'Goni''s safety argument can be stated plainly: This principle addresses several failure modes common to autonomous AI systems.'
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/05-sovereign-delegation-os-thesis.md
  heading: 12. Threat Model
  revision: c48031be8a4ca8ba57323dccb803c8dcdaab65dc
---

# 12. Threat Model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 12. Threat Model

Goni's safety argument can be stated plainly:

```text
The model is never automatically trusted with power.
```

This principle addresses several failure modes common to autonomous AI systems.
Model hallucination does not become action in the target architecture because
effects pass through authority mediation. Prompt injection does not become tool
authority because tools require capability-scoped mediation. Private memory does
not leak to
cloud systems because network egress is gated and receipt-linked by design.
Invisible agent behavior is constrained by reconstructable receipts by design.
Unbounded automation is controlled through mandates, corridors, budgets,
approval thresholds, and revocation.

Goni does not make models safe by trusting them. It makes them useful by
containing their authority.

This is the project's strongest academic safety position. It does not depend
on perfect model alignment. Instead, it assumes models are useful but fallible
components inside a governed system. The safety boundary is architectural, not
merely behavioral. The system trust posture is summarized in
[20-system/20-trust-model.md](/blueprint/20-system/20-trust-model.md).
