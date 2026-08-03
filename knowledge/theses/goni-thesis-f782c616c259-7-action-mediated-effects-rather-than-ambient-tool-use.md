---
id: GONI-THESIS-F782C616C259
title: '7. Action: Mediated Effects Rather Than Ambient Tool Use'
type: thesis
status: draft
implementation_state: specified_only
proposition: In ordinary agent frameworks, tools often function as model-callable plugins.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/05-sovereign-delegation-os-thesis.md
  heading: '7. Action: Mediated Effects Rather Than Ambient Tool Use'
  revision: c48031be8a4ca8ba57323dccb803c8dcdaab65dc
---

# 7. Action: Mediated Effects Rather Than Ambient Tool Use

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Action: Mediated Effects Rather Than Ambient Tool Use

In ordinary agent frameworks, tools often function as model-callable plugins.
Goni rejects that framing. Tools are closer to kernel-mediated syscalls. The
model may request an action, but execution flows through policy evaluation,
capability validation, risk assessment, and receipt generation. The technical
contract for this direction is described in
[tool-capability-api.md](/blueprint/30-specs/tool-capability-api.md).

This distinction is essential. A model that can call tools directly has ambient
authority. A model that passes through kernel mediation has bounded agency.
The former is an agent loop. The latter is an operating-system design.

A Goni action therefore includes a visible chain from user intent or
observed event, to Work Order, to policy check, to capability token, to mediated
execution, to receipt, to memory update. The minimum sovereign loop is:

```text
Work Order -> Policy Check -> Capability Token -> Mediated Action
-> Receipt -> Memory Update
```

This loop is the smallest proof that Goni is not merely an assistant. It
demonstrates that the system can transform a goal into an authorized effect
without allowing the model to own authority directly.
