---
id: GONI-SPEC-3DE3CA5BC51F
title: 2. TCB definition
type: specification
status: draft
implementation_state: specified_only
proposition: 'Minimum trusted computing base for mediation: policy engine and capability issuer, mediation gateway (agent syscall boundary), receipt root and integrity chain, egress gateway/proxy and decision logger.'
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
  heading: 2. TCB definition
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 2. TCB definition

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. TCB definition

Minimum trusted computing base for mediation:
- policy engine and capability issuer,
- mediation gateway (agent syscall boundary),
- receipt root and integrity chain,
- egress gateway/proxy and decision logger.

Untrusted by default:
- model runtimes,
- tool plugin code,
- connector adapters,
- user-defined automation scripts.
