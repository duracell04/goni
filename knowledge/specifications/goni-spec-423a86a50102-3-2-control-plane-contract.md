---
id: GONI-SPEC-423A86A50102
title: 3.2 Control-plane contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'Delegation is policy-first: the model proposes intent repair, plans, and tool actions, the pre-execution control plane compiles the Work Order and Done Contract, the kernel authorizes or denies execution, irreversible side effects require explicit approval or an approved two-phase commit path, every side effect emits a receipt with delegation metadata.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 3.2 Control-plane contract
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 3.2 Control-plane contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Control-plane contract

Delegation is policy-first:

- the model proposes intent repair, plans, and tool actions,
- the pre-execution control plane compiles the Work Order and Done Contract,
- the kernel authorizes or denies execution,
- irreversible side effects require explicit approval or an approved two-phase
  commit path,
- every side effect emits a receipt with delegation metadata.
