---
id: D-025
title: D-025 — Sovereign cooperative compute
type: decision
status: draft
implementation_state: specified_only
proposition: GONI keeps identity, memory, and authority under the principal while
  permitting policy-bounded cooperation in computation and shareable evidence.
domains:
- sovereignty
- compute
- kernel
aliases: []
relations:
- type: depends_on
  target: NET-01
- type: depends_on
  target: SS-01
- type: refines
  target: GONI-DECISION-E5B906550794
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: The disclosure boundary and protected-execution profiles require implementation
  and adversarial evaluation; sovereignty is a specified contract.
legacy: []
---

# D-025 — Sovereign cooperative compute

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

## Decision

GONI is sovereign locally and may cooperate computationally. The local kernel
retains the principal's identity, canonical private memory, policy, mandate,
capability, revocation, and state-acceptance authority.

\[
G_i=(Private_i,Capabilities_i),\qquad
Disclosure_i\subseteq AuthorizedExports_i.
\]

The export set is defined by the principal's canonical policy for a specific
recipient, purpose, trust domain, and validity interval. Private state remains
owner-governed wherever an authorized protected copy is processed.

## Rationale and alternatives

Capability cooperation can extend capacity while preserving the existing
delegation kernel. Local-only execution remains a complete permitted profile;
an owner mesh and independent cooperative providers are additional profiles
subject to the same authority boundary.

The original proposal's \(Private_i\not\subset Network\) is retained as derivation
context: it expresses withholding some private state, but does not establish a
per-export confidentiality boundary. The authorized-disclosure relation above
states the intended operational restriction more precisely.

## Consequences

NET-COMP-01 owns compute-export conformance. Existing NET-01 owns governed egress.
Computation verification and kernel acceptance retain separate responsibilities.
Cooperative participants receive the bounded execution terms and exported data
authorized for their role. Membership, capacity, or submitted evidence grants
only the powers recorded in canonical authority state.
