---
id: GONI-SYNTHESIS-VERIFIED-COMPUTE-SUBSTRATE
title: Verified Compute Substrate
type: synthesis
status: draft
implementation_state: specified_only
proposition: GONI can extend its Execution Plane with a policy-defined, evidence-carrying
  compute substrate that preserves delegation authority while making heterogeneous
  computation addressable through common contracts.
domains:
- compute
- kernel
- scheduler
- receipts
aliases:
- verified-compute-substrate
- UCT-01
relations:
- type: synthesizes
  target: GONI-PRINCIPLE-COMPUTE-SEMANTIC-NEUTRALITY
- type: synthesizes
  target: COMP-01
- type: synthesizes
  target: EXEC-01
- type: synthesizes
  target: GONI-PRINCIPLE-VERIFICATION-BOUNDARIES
- type: synthesizes
  target: EVID-01
- type: synthesizes
  target: RESULT-01
- type: synthesizes
  target: GONI-PRINCIPLE-QUALIFIED-PROVIDER-SUBSTITUTABILITY
- type: synthesizes
  target: DIST-01
sources: []
artifacts: []
uncertainty: This synthesis describes a research and architecture direction. It does
  not establish implementation, scalability, proof economics, or a global compute
  market.
legacy: []
---

# Verified Compute Substrate

GONI remains a sovereign Delegation OS. The verified compute substrate is a lower layer inside the Execution Plane, not a replacement for the authority kernel.

The architectural flow is:

[
Mandate
\rightarrow Work\ Order
\rightarrow Kernel\ Authority
\rightarrow Computation\ Specification
\rightarrow Execution\ Contract
\rightarrow Placement+Evidence\ Planning
\rightarrow Execution
\rightarrow Evidence
\rightarrow Result\ Receipt
\rightarrow State/Effect
]

Settlement is attached where economically relevant; it is not intrinsic to computation.

## Layer boundary

The kernel answers:

> May this work be performed, with these data, powers, constraints, and external effects?

The compute substrate answers:

> Given that authority, what computation is acceptable, where can it execute, what evidence must accompany it, and what result can the kernel accept?

This preserves GONI's existing doctrine:

> Models reason. The kernel authorizes. Tools act. Receipts prove what was authorized and observed.

The stronger compute formulation refines the last clause: receipts carry typed evidence and provenance; they do not magically turn every claim into cryptographic proof.

## Common contract, heterogeneous execution

The substrate should expose one contract family across heterogeneous CPU, GPU, NPU, proof-oriented, remote, or future specialized resources. It remains physically specialized and locality-aware.

The scheduler chooses among qualified routes using execution cost, evidence cost, verification cost, transfer/storage cost, expected failure cost, latency, energy, privacy, and sovereignty constraints.

## Research claim

The potentially valuable contribution is not that individual primitives are novel. Heterogeneous compilation, verifiable computation, distributed scheduling, provenance, and settlement all have prior art.

The GONI-specific research question is whether these can be composed beneath a sovereign delegation kernel so that:

1. application meaning remains governed;
2. computation becomes addressable through a common contract;
3. execution and assurance are planned jointly;
4. results carry typed evidence and canonical receipts;
5. provider substitution is possible within explicit service classes;
6. the full system improves end-to-end cost or assurance for meaningful workloads.

The next proof should be empirical: one contract interface, several computational profiles, multiple execution/evidence paths, and measured end-to-end tradeoffs.


## Universal Compute Transition (UCT-01)

UCT-01 names the composition of the existing compute contracts. In the proposed
program-execution profile, a compact explanatory envelope is:

\[
J=(P,I,S,\rho,\Sigma,B,\Pi,R),\qquad
Exec(J,H)\rightarrow(O,S',m,\pi).
\]

This notation summarizes the contracts below; it is not a second JobSpec or a
complete serialized execution descriptor. JOB-01 remains scheduler-visible work,
and one job can reference several computations. Relation-satisfaction mode is
defined separately in COMP-01.

| Envelope term | Authoritative home |
|---|---|
| Program, inputs, starting state, randomness, numerical semantics, computational bounds | [COMP-01](../specifications/comp-01-computation-specification.md) |
| Request identity, authority, delivery, deadline, request budget, privacy, settlement | [EXEC-01](../specifications/exec-01-execution-contract.md) |
| Required evidence policy, verifier, assumptions, and claim scope | [EVID-01](../specifications/evid-01-computation-evidence-contract.md) |
| Proposed output/state, meter references, delivery and acceptance decisions | [RESULT-01](../specifications/result-01-computation-result-receipt.md) |

Here \(\pi\) denotes typed attached evidence; a cryptographic proof is one supported
design class. Meter assurance is distinct from result correctness. \(S'\) denotes
a proposed computational state transition; kernel acceptance determines its
authoritative use. Execution lifecycle, failure, and cancellation remain governed
by the job and execution contracts.

The illustrative commitment \(C_J=Commit(P,I,S,\rho,\Sigma)\) identifies only the
listed computational core. COMP-01 binds its full accepted specification and
EXEC-01 separately binds per-request terms. The commitment mechanism's binding
and hiding properties are explicit design questions.
