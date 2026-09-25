---
id: DELEG-RESPONSIBILITY-01
title: Responsibility lineage under delegated execution
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni should distinguish execution, attribution, answerability, accountability, and legal or moral responsibility while preserving enough delegation provenance to reconstruct which principal grant, policy decision, delegate, authorization, effect, and verification path produced a consequential outcome.
domains: [delegation, accountability, receipts, governance]
aliases:
- responsibility lineage
- accountability chain
relations:
- type: refines
  target: GONI-THESIS-E9DC181C1E9A
- type: depends_on
  target: DELEG-MODEL-01
- type: synthesizes
  target: EVID-DELEGATION-RESPONSIBILITY-01
sources:
- SRC-KONG2026-DELEGATION-RESPONSIBILITY
artifacts: []
uncertainty: Technical provenance can support answerability and audit but does not itself settle moral responsibility, legal liability, organizational accountability, or jurisdiction-specific duties.
legacy: []
---

# Responsibility lineage under delegated execution

Delegation separates several concepts that should remain distinct:

- **execution** — which component physically or digitally produced the effect;
- **attribution** — which model, runtime, tool, identity, or external actor participated;
- **authorization** — which valid authority path permitted the effect;
- **answerability** — which human or organizational principal must be able to explain the delegation and its governing conditions;
- **accountability** — which review, remediation, sanction, or correction process applies;
- **legal or moral responsibility** — a normative conclusion determined outside the technical architecture.

Goni receipts should preserve the technical lineage required to ask these questions:

[
Principal
ightarrow Delegation
ightarrow WorkOrder
ightarrow Capability
ightarrow Delegate
ightarrow Authorization
ightarrow Effect
ightarrow Observation
ightarrow Verification
ightarrow Receipt
]

The receipt establishes reconstructable provenance. It does not automatically decide responsibility.
