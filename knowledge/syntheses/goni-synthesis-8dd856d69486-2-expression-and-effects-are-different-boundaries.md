---
id: GONI-SYNTHESIS-8DD856D69486
title: 2. Expression and effects are different boundaries
type: synthesis
status: draft
implementation_state: specified_only
proposition: The model may emit text.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/65-local-sovereign-knowledge-runtime.md
  heading: 2. Expression and effects are different boundaries
  revision: 43414875152ae18f9977f21c9786b2d7025081ac
---

# 2. Expression and effects are different boundaries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Expression and effects are different boundaries

The model may emit text. Text alone grants no authority. Authority begins when
the system attempts to commit a durable change or impose an effect outside the
ephemeral reasoning context.

| Operation | TOOL-01 capability required? | Reason |
| --- | --- | --- |
| Generate, analyze, criticize, summarize, or draft locally | No | Private computation and expression are not tool effects. |
| Propose a memory, file, message, payment, or device action | No, until commit | A proposal carries no execution authority. |
| Commit durable memory or ontology changes | Yes | Canonical owner-controlled state changes. |
| Write or delete files, send or publish content, move money, call a network service, or actuate a device | Yes | Durable or external effects cross the kernel boundary. |

The effect path is deliberately simple:

1. The agent generates or proposes.
2. Policy determines whether the requested effect is admissible.
3. The principal authorizes directly or through an explicitly delegated
   capability where the owner's policy requires it.
4. The kernel executes through a scoped tool.
5. Receipts make the material chain reconstructible.

Content classification does not create or remove tool authority. A harmless
sentence cannot authorize a payment, and an offensive sentence does not justify
silently expanding surveillance or blocking unrelated local computation.
