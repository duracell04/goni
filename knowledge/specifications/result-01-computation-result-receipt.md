---
id: RESULT-01
title: Computation Result Receipt
type: specification
status: draft
implementation_state: specified_only
proposition: Accepted computation results should extend GONI's canonical receipt system with bindings to the execution contract, computation identity, output and state references, evidence, metering, delivery status, and acceptance decision.
domains:
- compute
- receipts
- specs
aliases:
- result-receipt
relations:
- type: depends_on
  target: EXEC-01
- type: depends_on
  target: EVID-01
- type: refines
  target: REC-01
- type: refines
  target: GONI-SPEC-67C20FD1E8A7
sources:
- SRC-BOUNDLESS-PROOF-LIFECYCLE
artifacts: []
uncertainty: Exact receipt schema fields and storage references remain to be added to a machine-readable schema before implementation.
legacy: []
---

# Computation Result Receipt

GONI should reuse its canonical receipt architecture rather than create a parallel compute-market log.

A computation result receipt should carry or reference:

- `contract_id`;
- `computation_id`;
- `provider_or_executor_ref`;
- `output_ref` or output commitment;
- `starting_state_ref`;
- `resulting_state_ref` or state-delta reference;
- `evidence_policy_ref`;
- `evidence_refs`;
- `metering_refs`;
- `delivery_status`;
- `acceptance_decision`;
- `settlement_ref` when economic settlement applies.

The receipt records which claim was accepted under which contract. It does not turn all referenced evidence into the same assurance type.

## Acceptance is broader than proof verification

A valid proof or attestation may be necessary without being sufficient for contract completion. Acceptance can additionally require:

- the referenced starting state to remain admissible;
- required output data to be delivered or available;
- the requested privacy constraints to have been met;
- external side effects to return their own authenticated completion evidence;
- the payment authorization to remain valid and unsettled.

The existing GONI receipt remains the governance record. RESULT-01 adds computation-specific bindings to that record.
