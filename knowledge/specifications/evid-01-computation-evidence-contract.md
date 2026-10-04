---
id: EVID-01
title: Computation Evidence Contract
type: specification
status: draft
implementation_state: specified_only
proposition: Each execution contract should declare a typed evidence policy that defines the claim to establish, the accepted evidence mechanism, verifier, assumptions, freshness requirements, privacy properties, and composition rules.
domains:
- compute
- security
- specs
aliases:
- computation-evidence-policy
relations:
- type: depends_on
  target: EXEC-01
- type: depends_on
  target: GONI-PRINCIPLE-VERIFICATION-BOUNDARIES
- type: refines
  target: REC-01
sources:
- SRC-RISC0-ZKVM
- SRC-SP1-SECURITY
- SRC-BOUNDLESS-PROOF-LIFECYCLE
artifacts: []
uncertainty: The initial implementation may support only a subset of evidence types; cryptographic proof, hardware attestation, deterministic replay, and provider attestations carry different assumptions and should remain distinguishable.
legacy: []
---

# Computation Evidence Contract

A generic evidence field is not enough to state what has been established.

The execution contract should reference an evidence policy:

[
EvidencePolicy =
(claim, mechanism, verifier, assumptions, freshness, privacy, composition)
]

## Evidence mechanisms

The vocabulary may include:

- `cryptographic_proof`;
- `hardware_attestation`;
- `deterministic_replay`;
- `independent_reexecution`;
- `optimistic_challenge`;
- `trusted_provider_attestation`;
- `authenticated_telemetry`.

These mechanisms are typed rather than ranked on one universal security scale.

A hardware attestation and a zero-knowledge proof do not provide the same guarantee. Two agreeing workers do not necessarily constitute independent evidence. A signed provider receipt does not prove that the provider's program was correct. A hash binds data identity under stated assumptions but is not itself execution evidence.

## Required policy fields

A policy should define:

- `claim_type`: exactly what proposition is being checked;
- `mechanism`: the evidence class;
- `verifier_ref`: approved verifier, measurement root, or trust anchor;
- `bound_artifacts`: program, input, state, output, and environment identities relevant to the claim;
- `assumptions`: accepted cryptographic, hardware, provider, or challenge assumptions;
- `freshness`: whether old evidence is reusable and which nonce, epoch, or timestamp constraints apply;
- `privacy`: what the evidence hides from which party;
- `composition`: how this evidence may be combined with evidence from other execution boundaries.

## Privacy boundary

Zero knowledge concerns what a proof reveals to its verifier. It does not, by itself, guarantee that an outsourced worker never observed the private input. Confidential execution therefore remains a separate placement and trust requirement unless the evidence mechanism explicitly covers it.
