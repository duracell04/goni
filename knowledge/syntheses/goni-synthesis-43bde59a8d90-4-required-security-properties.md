---
id: GONI-SYNTHESIS-43BDE59A8D90
title: 4. Required security properties
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Complete mediation: no side effect without policy check.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/20-trust-model.md
  heading: 4. Required security properties
  revision: 628398028a2ae5fe5696b6b3ec004da2314ddd96
---

# 4. Required security properties

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Required security properties

- Complete mediation: no side effect without policy check.
- Least privilege: capabilities are scoped, attenuable, and expire.
- Confinement by default: no ambient network authority.
- Auditability: mediated actions emit verifiable receipts.
- Revocability: authority and extension trust can be withdrawn quickly.
