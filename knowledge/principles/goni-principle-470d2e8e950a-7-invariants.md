---
id: GONI-PRINCIPLE-470D2E8E950A
title: 7. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: 'No ambient authority: tools cannot perform privileged action without a valid capability handle.'
domains:
- agent
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md
  heading: 7. Invariants
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 7. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Invariants

- No ambient authority: tools cannot perform privileged action without a valid
  capability handle.
- No silent effect: every effectful action yields a receipt reference.
- No invisible policy: allow/deny basis is machine-readable.
- No duplicate mutation: idempotency keys prevent repeated side effects.
- No outsourced kernel: external runtimes/gateways must call into the ABI for
  effectful work; they may not redefine authority, receipt semantics, or commit
  rules outside it.
