---
id: GONI-DECISION-62ACE8B32A6F
title: D-015 - Deterministic inference preset for audit/self-loop workloads
type: decision
status: draft
implementation_state: specified_only
proposition: The Execution plane exposes a deterministic preset as one explicitly scoped execution-semantics profile rather than treating a seed alone as a reproducibility guarantee.
domains:
- software
- compute
aliases: []
relations:
- type: refines
  target: COMP-01
  note: D-015 is a concrete deterministic profile within the broader COMP-01 execution-semantics contract.
sources:
- SRC-PYTORCH-DETERMINISM
artifacts: []
uncertainty: Backend-specific reproducibility guarantees vary by hardware, driver, runtime, and operation; exact supported profiles require implementation evidence.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-015 - Deterministic inference preset for audit/self-loop workloads
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-015 - Deterministic inference preset for audit/self-loop workloads

> Status boundary: this remains a specified-only design. Backend reproducibility must be measured and documented before stronger claims are made.

## Formal statement

The Execution plane exposes a deterministic preset as a declared `execution_semantics` profile. A request marked deterministic binds the reproducibility scope and, where supported:

- temperature = 0 for decoding paths where sampling is unnecessary;
- fixed or externally committed RNG state when randomness is part of the computation;
- batch size = 1 with continuous or dynamic batching disabled where batching can alter results;
- constrained worker/thread behavior where concurrency changes operation order;
- deterministic backend algorithms where available;
- numerical modes relevant to reproducibility, including TF32 policy where applicable;
- model, runtime, compiler, driver, and material hardware-profile hashes or versions.

A seed alone is not treated as a sufficient reproducibility guarantee. The contract states the environment and semantic scope within which repeatability is expected.

## Rationale

Self-loop and agent chains can amplify small numerical or token-level differences. Audited runs may therefore value reproducibility more highly than maximum throughput. PyTorch's deterministic-algorithm controls also illustrate the narrower guarantee: deterministic operation selection is useful, while full application reproducibility depends on additional environment and execution conditions.

## Consequences

- Engines should expose a slower deterministic profile rather than silently weakening a deterministic request.
- CI should test the declared reproducibility property for supported profiles.
- Fast defaults may use batched or accelerator-specific paths while the audit profile remains separately addressable.
- Approximate or cross-platform profiles should declare their comparison relation and tolerance rather than being described as bitwise deterministic.
