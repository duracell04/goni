---
id: GONI-SPEC-7D3B2848B0BF
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: No outbound network bytes occur unless issued through tool_call with an authorized net capability.
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
  heading: Conformance tests
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- No outbound network bytes occur unless issued through `tool_call` with an
  authorized net capability.
- Every successful, denied, failed, and cancelled `tool_call` emits one receipt.
- Mutating calls without `idempotency_key` are rejected.
- Cancellation behavior matches declared cancellation policy.
