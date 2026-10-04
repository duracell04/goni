---
id: NET-COMP-01
title: Cooperative compute export
type: specification
status: draft
implementation_state: specified_only
proposition: A cooperative compute request exports a recipient-scoped computation
  and execution contract through existing governed egress, subject to an explicit
  disclosure policy and local result-acceptance authority.
domains:
- network
- privacy
- compute
aliases: []
relations:
- type: depends_on
  target: D-025
- type: depends_on
  target: NET-01
- type: depends_on
  target: EXEC-01
- type: depends_on
  target: EVID-01
- type: depends_on
  target: RESULT-01
- type: depends_on
  target: GONI-SPEC-C237E3663D04
- type: refines
  target: GONI-SPEC-8AF8699481AF
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Leakage metrics, confidentiality mechanisms, recipient authentication,
  and cumulative disclosure accounting require profile-specific implementation and
  evidence.
legacy: []
---

# Cooperative compute export

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

## Trust domains

OwnerMesh and CooperativeNetwork are distinct authority domains even when they
share a transport or physical host. Device ownership and explicit enrollment
determine owner-mesh membership. A peer or provider's network reachability is
separate from its granted powers. Owner-mesh enrollment and cooperative requests
use the existing principal, capability, revocation, and egress mechanisms.

## Export contract

\[
J_{net}=ExportPolicy(J_{private},recipient,purpose,policy),\qquad
Disclosure(J_{net})\subseteq AuthorizedExports(policy).
\]

For each request a conforming export compiler MUST resolve the input scope,
recipient/trust class, authority and expiry, computational specification,
execution contract, permitted output/evidence disclosures, and resource bounds.
It MUST emit the least disclosure required by the permitted execution profile,
including any authorized protected representation. Unknown or unsatisfied export
requirements terminate that route; the scheduler may select a permitted local
route or surface the unresolved constraint.

Payloads, prompts, model/adaptor state, references, commitments, receipts,
telemetry, and observable metadata are included in export review. A protected
input profile states what the worker, verifier, operator, and other peers can
observe. Hash commitments to low-entropy inputs need a separately specified
hiding mechanism when the disclosure policy requires confidentiality.

## Leakage claim

\[
Leakage_{\mathcal A,\mathcal M}(J_{net},history)\le L_{policy}.
\]

This is a scoped conformance objective. A profile claiming the bound MUST name
the adversary \(\mathcal A\), observable view, leakage measure \(\mathcal M\), units,
composition rule across requests, and assumptions. Where a numeric measure is
unavailable, the profile MUST state explicit permitted disclosures and unresolved
leakage rather than substituting an invented bound. ExportPolicy is a deterministic
policy boundary; semantic redaction alone supplies candidate transformations.

## Return path

Returned outputs and evidence enter EXEC-01/EVID-01 acceptance and RESULT-01
receipts. The local kernel checks current authority and starting-state
compatibility before adopting a state change or mediating an external effect.
Revocation, timeout, cancellation, delivery failures, and unsettled payment state
use the existing job and execution lifecycle. Receipt access and export retain
the existing owner-facing purpose and privacy limits.
