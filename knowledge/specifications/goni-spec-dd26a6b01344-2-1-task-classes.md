---
id: GONI-SPEC-DD26A6B01344
title: 2.1 Task classes
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every mediated action MUST be tagged with a task_class (for example: email_reply, invoice_payment, calendar_change, doc_edit).'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 2.1 Task classes
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 2.1 Task classes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Task classes

Every mediated action MUST be tagged with a `task_class` (for example:
`email_reply`, `invoice_payment`, `calendar_change`, `doc_edit`).
