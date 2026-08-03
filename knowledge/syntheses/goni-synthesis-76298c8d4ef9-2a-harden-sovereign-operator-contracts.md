---
id: GONI-SYNTHESIS-76298C8D4EF9
title: 2a) Harden sovereign-operator contracts
type: synthesis
status: draft
implementation_state: specified_only
proposition: Audit existing Work Order, Done Contract, receipt, CapabilityToken, AutonomyCorridor, memory retrieval, and model-registry specs for one end-to-end governed action path.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/30-next-steps.md
  heading: 2a) Harden sovereign-operator contracts
  revision: 050465b8d1a68fe8cc36e542344414705c3e08a7
---

# 2a) Harden sovereign-operator contracts

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2a) Harden sovereign-operator contracts
- Audit existing Work Order, Done Contract, receipt, CapabilityToken,
  AutonomyCorridor, memory retrieval, and model-registry specs for one
  end-to-end governed action path.
- Ensure the path preserves:
  - Work Order reference,
  - Done Contract hash,
  - capability token,
  - policy decision,
  - sandbox class,
  - receipt tier,
  - rollback reference where applicable.
- Add trace fixtures for at least one reversible write and one hard-gated
  external side effect.
