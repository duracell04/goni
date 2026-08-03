---
id: GONI-SPEC-4D10349C481A
title: 3. Idempotency rules
type: specification
status: draft
implementation_state: specified_only
proposition: 'For any non-read-only operation: idempotency_key is mandatory, replay of identical tuple (action, args_hash, capability_id, idempotency_key) MUST return the original outcome, replay with mismatched tuple MUST be rejected and logged.'
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
  heading: 3. Idempotency rules
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 3. Idempotency rules

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Idempotency rules

For any non-read-only operation:
- `idempotency_key` is mandatory,
- replay of identical tuple
  (`action`, `args_hash`, `capability_id`, `idempotency_key`) MUST return the
  original outcome,
- replay with mismatched tuple MUST be rejected and logged.
