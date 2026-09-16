---
id: GONI-THESIS-18044634B6FC
title: Retrieval Is Not Authorization
type: thesis
status: draft
implementation_state: specified_only
proposition: Increasing the information available to a model, or replacing it with a more capable model, must not by itself enlarge the authority granted to the delegated system.
domains:
- authority
- memory
- architecture
aliases: []
relations:
- type: refines
  target: GONI-THESIS-E1FB8B4F7772
- type: depends_on
  target: GONI-SPECIFICATION-BB656A72DF7D
sources: []
artifacts: []
uncertainty: This is a normative architectural constraint. Runtime non-bypassability requires separate implementation and test evidence.
legacy: []
---

# Retrieval Is Not Authorization

GONI separates epistemic capability from delegated authority. A model may receive more evidence, a larger context, a retrieved source, a stronger inference backend, or a cloud escalation while remaining inside exactly the same mandate and capability boundaries.

The intended invariants are:

`Δcontext ↛ Δauthority`

`Δmodel_capability ↛ Δauthority`

Accordingly, paging a memory item into active context does not grant tool rights; higher retrieval confidence does not bypass policy mediation; a larger local or remote model does not receive a wider capability token merely because it can reason better; and cloud escalation does not automatically inherit unrestricted access to local memory or network egress.

This distinction is necessary because cognitive residency, model capability, and execution authority solve different system problems. Collapsing them would allow a performance optimization to become an implicit privilege escalation.

The compact design rule is:

**Model capability determines reasoning capacity. The kernel determines authority.**

This thesis specializes the broader GONI system architecture in which models reason, the kernel authorizes, tools act, and receipts prove.
