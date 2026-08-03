---
id: GONI-SYNTHESIS-E3268E178B89
title: 2. What already exists
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'The audit is directionally right, but the repo already contains partial P0 contracts: Work Orders and Done Contracts: Delegation interface.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/55-sovereign-operator-audit-gap-map.md
  heading: 2. What already exists
  revision: 42acf7b164bf9f71154d2bf6c242e753fc43b714
---

# 2. What already exists

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. What already exists

The audit is directionally right, but the repo already contains partial P0
contracts:

- Work Orders and Done Contracts:
  [Delegation interface](/blueprint/30-specs/delegation-interface.md).
- Receipts:
  [Receipts](/blueprint/30-specs/receipts.md).
- Capability tokens and mediated tools:
  [Tool Capability API](/blueprint/30-specs/tool-capability-api.md).
- Autonomy corridors:
  [Delegation and autonomy](/blueprint/30-specs/delegation-and-autonomy.md).
- Governed memory retrieval:
  [Memory retrieval](/blueprint/30-specs/memory-retrieval.md).
- Model provenance and bundle governance:
  [Model registry](/blueprint/30-specs/model-registry.md).
- Kernel-owned authority over external gateways:
  [Software decision D-023](/blueprint/software/90-decisions.md).

Therefore the immediate work is not to invent these objects from scratch. It is
to harden them, connect them, and add missing taxonomy where the audit exposes
underspecified behavior.
