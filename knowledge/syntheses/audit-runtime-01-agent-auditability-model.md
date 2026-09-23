---
id: AUDIT-RUNTIME-01
title: Agent auditability model
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni receipts and runtime evidence should be evaluated for action recoverability, lifecycle coverage, policy checkability, responsibility attribution, and evidence integrity rather than treated as ordinary application logs.
domains:
- system
- audit
- provenance
aliases:
- auditability model
relations:
- type: refines
  target: GONI-THESIS-E9DC181C1E9A
sources:
- SRC-NIAN2026-AUDITABLE-AGENTS
- SRC-W3C2013-PROV
- SRC-W3C2021-TRACE-CONTEXT
artifacts: []
uncertainty: This node maps an external auditability taxonomy onto Goni's receipt doctrine; full compliance with each dimension requires workflow-specific evidence and is not claimed by specification alone.
legacy: []
---

# Agent auditability model

Goni distinguishes **logging**, **receipts**, **auditability**, and **auditing**.

- **Logs** support operations and debugging.
- **Receipts** are governance artifacts for consequential state transitions.
- **Auditability** is the system property that makes later reconstruction and policy assessment possible.
- **Auditing** is the process of examining trustworthy evidence after or during execution.

The five auditability dimensions from Nian et al. map naturally onto Goni:

| Dimension | Goni interpretation |
| --- | --- |
| Action recoverability | Reconstruct what action or state transition actually occurred. |
| Lifecycle coverage | Preserve evidence from objective and authorization through execution, observation, verification, and final state. |
| Policy checkability | Recover the policy, mandate, capability, and relevant version or hash used for the decision. |
| Responsibility attribution | Identify the principal, runtime/model/tool components, authorization path, and external actors involved. |
| Evidence integrity | Make tampering, omission, or ambiguity in consequential evidence detectable where feasible. |

A useful consequential-action trace is therefore:

```text
objective
-> evidence/context references
-> model/runtime decision
-> requested capability
-> authorization decision
-> actual tool execution
-> resulting observation
-> verification result
-> state transition
-> receipt
```

This extends Goni's existing receipt doctrine with an explicit evaluation model for whether receipts are sufficient for accountability.
