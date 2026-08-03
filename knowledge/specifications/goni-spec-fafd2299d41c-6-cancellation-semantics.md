---
id: GONI-SPEC-FAFD2299D41C
title: 6. Cancellation semantics
type: specification
status: draft
implementation_state: specified_only
proposition: 'The ABI supports three cancellation points: immediate: cancel before any effectful substep starts, after_token: cancel at model token boundary, after_tool_call: cancel once current mediated tool action completes.'
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
  heading: 6. Cancellation semantics
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 6. Cancellation semantics

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Cancellation semantics

The ABI supports three cancellation points:
- `immediate`: cancel before any effectful substep starts,
- `after_token`: cancel at model token boundary,
- `after_tool_call`: cancel once current mediated tool action completes.

Cancellation policy is declared on job submission and logged in receipts.
