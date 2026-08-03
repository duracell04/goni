---
id: GONI-SPEC-31F371FDA554
title: 5. Credential and secret handling
type: specification
status: draft
implementation_state: specified_only
proposition: Tool runners must not receive long-lived cloud credentials by default.
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-ENF-01-non-bypassable-mediation.md
  heading: 5. Credential and secret handling
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 5. Credential and secret handling

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Credential and secret handling

- Tool runners must not receive long-lived cloud credentials by default.
- Secrets are requested via scoped handles and short-lived grants.
- Any secret-bound external call must be policy-mediated and receipt-linked.
