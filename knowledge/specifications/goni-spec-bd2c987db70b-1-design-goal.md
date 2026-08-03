---
id: GONI-SPEC-BD2C987DB70B
title: 1. Design goal
type: specification
status: draft
implementation_state: specified_only
proposition: 'The ABI makes complete mediation testable: no effectful operation is valid unless submitted through this interface, all approved/denied operations emit receipts, capabilities and policy decisions are explicit ABI inputs.'
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
  heading: 1. Design goal
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 1. Design goal

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Design goal

The ABI makes complete mediation testable:
- no effectful operation is valid unless submitted through this interface,
- all approved/denied operations emit receipts,
- capabilities and policy decisions are explicit ABI inputs.
