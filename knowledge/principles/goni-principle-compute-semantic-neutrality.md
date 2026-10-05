---
id: GONI-PRINCIPLE-COMPUTE-SEMANTIC-NEUTRALITY
title: Application-neutral execution with preserved semantics
type: principle
status: draft
implementation_state: specified_only
proposition: GONI should separate application-level meaning from execution-level operation semantics so identical computational operations can share one execution substrate without discarding correctness, authority, privacy, or acceptance semantics.
domains:
- compute
- kernel
- scheduler
aliases:
- semantic-neutrality
relations:
- type: refines
  target: GONI-SPEC-3C0787853B81
  note: The existing action/tool/model taxonomy remains a governance taxonomy while execution planning gains a lower-level operation taxonomy.
- type: depends_on
  target: GONI-SPEC-C2C63462FEAE
sources:
- SRC-MLIR-OFFICIAL
artifacts: []
uncertainty: This is a specified architectural principle. The concrete execution IR, operation taxonomy, and lowering stack remain open design choices.
legacy: []
---

# Application-neutral execution with preserved semantics

GONI should distinguish two different classification problems.

## Governance semantics

The kernel still needs to know what an operation means in the delegated system. Existing categories such as `action`, `tool`, and `model` remain useful because authority, privacy, approval, rollback, and receipt requirements depend on them.

## Execution semantics

The execution substrate should classify the actual computational work independently of the application that requested it. Candidate classes include:

- scalar and control-flow work;
- tensor and matrix work;
- vector work;
- hash and bitwise work;
- finite-field or proof-oriented arithmetic;
- memory and storage movement;
- network transfer.

A matrix multiplication does not become a different low-level operation because it originated in an LLM rather than a scientific simulation. Likewise, a hash operation does not need a different execution identity because it originated in a blockchain-facing workflow.

Application neutrality does not imply semantic neutrality in the correctness sense. The runtime must preserve every property that affects acceptable results, authority, security, privacy, resource placement, and cost.

The intended boundary is therefore:

> application-neutral execution planning, with semantics-preserving governance and acceptance contracts.
