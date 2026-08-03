---
id: GONI-SPEC-06EEDDD84AB3
title: 7. Retry policy
type: specification
status: draft
implementation_state: specified_only
proposition: 'Retries are allowed only when: idempotency is guaranteed by caller and target, retry budget permits, policy permits re-attempt class.'
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
  heading: 7. Retry policy
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 7. Retry policy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Retry policy

Retries are allowed only when:
- idempotency is guaranteed by caller and target,
- retry budget permits,
- policy permits re-attempt class.

Blind retries without idempotency evidence are forbidden.
