---
id: EXEC-01
title: Execution Contract
type: specification
status: draft
implementation_state: specified_only
proposition: GONI should bind each procurement or delegated execution request to a distinct execution contract that references a computation identity and separately defines authority, evidence, privacy, delivery, deadline, budget or price, settlement, and replay-control terms.
domains:
- compute
- kernel
- specs
aliases:
- compute-execution-contract
relations:
- type: depends_on
  target: COMP-01
- type: refines
  target: GONI-SPEC-D89F04D1E90A
  note: The kernel job descriptor can reference one or more execution contracts without losing its governance fields.
- type: depends_on
  target: GONI-SPEC-841BF8F599B9
sources: []
artifacts: []
uncertainty: The concrete signature format, offer/acceptance protocol, provider discovery mechanism, and settlement rail remain unspecified.
legacy: []
---

# Execution Contract

The computation identity and the agreement to perform or deliver that computation are different objects.

A conceptual execution contract is:

[
X = (
contract\_id,
computation\_id,
requester,
authorization,
evidence\_policy,
privacy\_policy,
delivery\_terms,
deadline,
budget\_or\_price,
settlement\_rule,
nonce,
idempotency
)
]

The contract is the GONI-facing agreement under which an authorized requester asks a local engine, remote provider, or future compute market to deliver an acceptable result.

## Identity separation

`computation_id` answers:

> What computational relation or approved program execution is being requested?

`contract_id` answers:

> Under which authority, assurance, delivery, and economic terms is this particular request being made?

The same accepted result may be reusable across multiple contracts when policy permits. Reuse of a computational result must not imply reuse of a payment authorization.

## Authority and replay control

The execution contract should bind, directly or by reference:

- the requester or mandate authority;
- the relevant capability/policy decision;
- expiry or deadline;
- a unique nonce or request identity;
- settlement idempotency;
- allowed provider or trust classes where applicable.

These fields connect the compute substrate to GONI's existing delegation kernel rather than creating a parallel authority system.

## Acceptance boundary

Successful computational verification is only one possible completion condition. Contract acceptance may additionally require:

- the expected starting state to remain admissible;
- required output data to be delivered or made accessible;
- the requested privacy and evidence policy to be satisfied;
- external side effects to return their own authenticated completion evidence;
- settlement to remain authorized and unsettled.

Payment remains a policy attached to accepted work, not an intrinsic property of computation.
