---
id: GONI-EXPERIMENT-0A35259EE5EA
title: 7. Evaluation limits
type: experiment
status: draft
implementation_state: not_applicable
proposition: Local evaluation receipts are attestations, not proofs.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: 7. Evaluation limits
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# 7. Evaluation limits

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Evaluation limits

Local evaluation receipts are attestations, not proofs. They can show which
tests ran, under which environment, against which model hash, with which
results. For adapters, they can show bounded before/after behavior under a
named eval pack. They do not prove absence of backdoors, lawful training data,
future safety, semantic equivalence to upstream claims, or universal alignment
with a user's intent.

Distributed trust remains a separate problem: one Goni node SHOULD NOT accept
another node's eval receipt without signature validation, evaluator identity,
environment disclosure, failure disclosure, and policy-approved reputation or
attestation rules. SLSA, in-toto, SPDX, and CycloneDX are relevant source
patterns for this supply-chain evidence model. [[slsa-framework]]
[[in-toto-framework]] [[spdx-overview]] [[cyclonedx-mlbom]]
