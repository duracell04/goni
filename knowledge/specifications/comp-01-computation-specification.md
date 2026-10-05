---
id: COMP-01
title: Computation Specification
type: specification
status: draft
implementation_state: specified_only
proposition: An authorized computation should be represented by a canonical computation specification that binds the program or relation, inputs, starting state, randomness policy, execution semantics, resource bounds, and acceptance relation independently of settlement terms.
domains:
- compute
- kernel
- specs
aliases:
- computation-contract
relations:
- type: refines
  target: JOB-01
  note: JOB-01 remains the scheduler-visible delegation job; COMP-01 defines the computation carried by a job.
- type: depends_on
  target: GONI-PRINCIPLE-COMPUTE-SEMANTIC-NEUTRALITY
- type: depends_on
  target: GONI-DECISION-62ACE8B32A6F
sources:
- SRC-MLIR-OFFICIAL
- SRC-PYTORCH-DETERMINISM
artifacts: []
uncertainty: Canonical serialization, commitment primitives, and the exact boundary between executable-program mode and relation-satisfaction mode remain to be specified.
legacy: []
---

# Computation Specification

A GONI Work Order or scheduler-visible Job may contain zero, one, or multiple computation specifications. The computation specification is below the delegation layer: it describes what computational result is acceptable, not whether the user has authorized the surrounding task.

A canonical model is:

[
CSpec = (M, P, \Phi, I, S, \rho, \Sigma, B, A)
]

where:

- `M` is the execution mode;
- `P` is an approved executable program or program identity when program execution is required;
- `\Phi` is an acceptance relation when satisfying a relation is sufficient;
- `I` binds input data or input commitments;
- `S` binds the starting state or state reference;
- `\rho` defines the randomness policy;
- `\Sigma` defines execution semantics;
- `B` defines resource and placement bounds relevant to acceptable execution;
- `A` defines the output and state-transition acceptance relation.

## Execution modes

Two modes are explicitly different:

1. `program_execution`: execute an approved program under the declared semantics.
2. `relation_satisfaction`: return a result that satisfies an approved relation, allowing alternative implementations when policy permits.

The second mode permits optimization or provider-specific algorithms without pretending that the same physical instruction trace was executed.

## Randomness policy

A random seed is one possible input to `\rho`; it is not, by itself, a complete reproducibility contract. The specification may also bind:

- RNG family and version;
- seed source or externally committed randomness;
- randomness-consumption rules where material;
- software and backend versions;
- declared reproducibility scope.

Where unbiased randomness matters, the policy must define who can choose or influence the randomness and at what point it becomes committed.

## Execution semantics

`\Sigma` may bind:

- precision and numeric formats;
- deterministic or tolerance-based semantics;
- allowed kernels or backend classes;
- compiler/runtime references;
- hardware profile constraints;
- comparison norm and tolerance when approximate equality is acceptable.

A tolerance must define what is compared, against which reference, and at what stage of the computation.

## Computation identity

A computation identity may be derived from a canonical encoding of the specification:

[
computation\_id = H(encode(CSpec))
]

The binding mechanism must state its privacy properties. A plain hash can identify content but does not automatically hide low-entropy private input.

The computation identity is intentionally independent of payment, provider, and per-request settlement terms. Those belong to EXEC-01.
