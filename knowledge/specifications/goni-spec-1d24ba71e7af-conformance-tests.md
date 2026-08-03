---
id: GONI-SPEC-1D24BA71E7AF
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: Runtime rejects a bundle with a mismatched manifest hash.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: Conformance tests
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests

- Runtime rejects a bundle with a mismatched manifest hash.
- Runtime rejects private-memory use when permission is `deny`.
- Runtime rejects sensitive-memory use below the configured assurance floor.
- Bundle installation emits an InstallReceipt with bundle and manifest hashes.
- Bundle promotion emits an EvalReceipt reference and RollbackRef.
- Adapter promotion emits an AdaptationReceipt with base bundle refs, adapter
  hashes, dataset ledger refs, eval refs, approval state, and AdapterRollbackRef.
- Runtime rejects undeclared adapters or model stacks with mismatched adapter
  hashes.
- Route selection can explain which base bundle, adapters, prompt/policy bundle,
  memory refs, and eval refs made the stack eligible for the task class.
- Route selection can explain why a bundle was eligible for the task class.
- Local eval receipts state test environment, model hash, result summary, and
  limits of inference.
- Visual routes reject bundles without required visual capabilities, allowed
  asset class, license state, workflow runtime provenance, and visual eval
  coverage for the requested task class.
