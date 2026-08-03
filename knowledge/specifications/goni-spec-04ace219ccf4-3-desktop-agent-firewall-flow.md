---
id: GONI-SPEC-04ACE219CCF4
title: 3. Desktop Agent Firewall flow
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every desktop, browser, or vision-mediated action follows this logical flow: The flow may stop at any stage.'
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 3. Desktop Agent Firewall flow
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 3. Desktop Agent Firewall flow

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Desktop Agent Firewall flow

Every desktop, browser, or vision-mediated action follows this logical flow:

```text
desktop/window/app/event
-> observation permission
-> extraction permission
-> memory permission
-> tool/action permission
-> autonomy corridor
-> receipt
```

The flow may stop at any stage. A permissioned-view assistant may stop after
observation and annotation. A memory layer may stop after memory write. An
agentic operator may continue to act only when actuation is separately granted.

Denied boundary transitions MUST fail closed and remain auditable.
