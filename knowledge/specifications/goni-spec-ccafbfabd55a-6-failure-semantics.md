---
id: GONI-SPEC-CCAFBFABD55A
title: 6. Failure semantics
type: specification
status: draft
implementation_state: specified_only
proposition: 'The kernel classifies failures as: policy_denied precondition_failed timeout partial_external_effect infra_error Each failure class must map to: retry policy, compensation requirement, escalation requirement, receipt outcome code.'
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md
  heading: 6. Failure semantics
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 6. Failure semantics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Failure semantics

The kernel classifies failures as:
- `policy_denied`
- `precondition_failed`
- `timeout`
- `partial_external_effect`
- `infra_error`

Each failure class must map to:
- retry policy,
- compensation requirement,
- escalation requirement,
- receipt outcome code.
