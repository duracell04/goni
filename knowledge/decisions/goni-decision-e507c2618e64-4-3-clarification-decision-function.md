---
id: GONI-DECISION-E507C2618E64
title: 4.3 Clarification decision function
type: decision
status: draft
implementation_state: specified_only
proposition: 'The runtime MUST choose: assume when the task is reversible or low-risk and surfaced assumptions are sufficient, ask_decisive when one answer materially changes plan, risk, tool choice, irreversibility, or audit scope, propose_objectives only in co_creation mode, block when safe execution is impossible under current policy and context.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 4.3 Clarification decision function
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 4.3 Clarification decision function

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.3 Clarification decision function

The runtime MUST choose:

- `assume` when the task is reversible or low-risk and surfaced assumptions are
  sufficient,
- `ask_decisive` when one answer materially changes plan, risk, tool choice,
  irreversibility, or audit scope,
- `propose_objectives` only in `co_creation` mode,
- `block` when safe execution is impossible under current policy and context.

The runtime MUST NOT ask questions that can be answered from:

- active policy,
- retrieved context,
- prior approvals,
- deterministic task constraints,
- or stable user defaults.
