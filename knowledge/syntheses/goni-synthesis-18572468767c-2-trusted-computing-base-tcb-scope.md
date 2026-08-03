---
id: GONI-SYNTHESIS-18572468767C
title: 2. Trusted computing base (TCB) scope
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Current intended TCB components: capability and policy decision path, tool mediation boundary, egress gate, receipt integrity path (hash chain and verification), minimal scheduler controls tied to mediation guarantees.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/20-trust-model.md
  heading: 2. Trusted computing base (TCB) scope
  revision: 628398028a2ae5fe5696b6b3ec004da2314ddd96
---

# 2. Trusted computing base (TCB) scope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Trusted computing base (TCB) scope

Current intended TCB components:
- capability and policy decision path,
- tool mediation boundary,
- egress gate,
- receipt integrity path (hash chain and verification),
- minimal scheduler controls tied to mediation guarantees.

Everything else is outside the TCB by default.
