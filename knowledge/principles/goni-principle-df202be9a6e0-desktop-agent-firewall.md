---
id: GONI-PRINCIPLE-DF202BE9A6E0
title: Desktop Agent Firewall
type: principle
status: draft
implementation_state: specified_only
proposition: The Desktop Agent Firewall is a kernel concept, not a preference panel.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/15-delegation-doctrine.md
  heading: Desktop Agent Firewall
  revision: 8eca78baa6e9fe022fe69ba0f6249f53ea9fa79b
---

# Desktop Agent Firewall

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Desktop Agent Firewall

The Desktop Agent Firewall is a kernel concept, not a preference panel. It
mediates whether an observed desktop, browser, app, or event can become
extracted context, durable memory, model input, synthetic input, external
egress, or a user-visible side effect.

Canonical flow:

```text
desktop/window/app/event
-> observation permission
-> extraction permission
-> memory permission
-> tool/action permission
-> autonomy corridor
-> receipt
```

The same model applies whether the upstream system is read-only, memory-only,
or fully agentic. The decisive boundary is not whether an assistant is local or
cloud-hosted; it is whether each power is mediated separately by the Goni
kernel.
