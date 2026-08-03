---
id: GONI-SPEC-14AA0EDAFDD4
title: 2. Atomicity boundary
type: specification
status: draft
implementation_state: specified_only
proposition: 'The kernel guarantees atomicity only within mediated local commit scopes: state deltas written through kernel-controlled stores, receipt append operations.'
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
  heading: 2. Atomicity boundary
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 2. Atomicity boundary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Atomicity boundary

The kernel guarantees atomicity only within mediated local commit scopes:
- state deltas written through kernel-controlled stores,
- receipt append operations.

External side effects (third-party APIs, email delivery, payment rails) are
not guaranteed reversible and require compensation policy.
