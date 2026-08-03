---
id: GONI-SYNTHESIS-9D368CF4D6E2
title: P0 - Kernel contract hardening
type: synthesis
status: draft
implementation_state: specified_only
proposition: Keep Work Order, Done Contract, Receipt, CapabilityToken, AutonomyCorridor, MemoryClass, ModelManifest, InstallReceipt, EvalReceipt, and RollbackRef as Goni-owned concepts.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/55-sovereign-operator-audit-gap-map.md
  heading: P0 - Kernel contract hardening
  revision: 42acf7b164bf9f71154d2bf6c242e753fc43b714
---

# P0 - Kernel contract hardening

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### P0 - Kernel contract hardening

- Keep Work Order, Done Contract, Receipt, CapabilityToken, AutonomyCorridor,
  MemoryClass, ModelManifest, InstallReceipt, EvalReceipt, and RollbackRef as
  Goni-owned concepts.
- Ensure mutating work preserves the chain:
  `intent -> Work Order -> Done Contract -> capability -> policy decision ->
  execution -> receipt -> rollback reference`.
- Ensure third-party framework logs never replace Goni receipts.
