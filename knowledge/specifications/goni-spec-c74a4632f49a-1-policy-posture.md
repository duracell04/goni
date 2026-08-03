---
id: GONI-SPEC-C74A4632F49A
title: 1. Policy posture
type: specification
status: draft
implementation_state: specified_only
proposition: 'Policy is default-deny: all effectful operations are denied unless explicitly authorized, all data downgrades require explicit declassification rules, all policy decisions are machine-readable and receipt-linked.'
domains:
- kernel
- policy
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-POL-01-policy-language.md
  heading: 1. Policy posture
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 1. Policy posture

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Policy posture

Policy is default-deny:
- all effectful operations are denied unless explicitly authorized,
- all data downgrades require explicit declassification rules,
- all policy decisions are machine-readable and receipt-linked.
