---
id: GONI-SPEC-136D62961150
title: 4.1 Delegation vs co-creation classification
type: specification
status: draft
implementation_state: specified_only
proposition: 'The runtime MUST choose delegation when: the objective is recoverable from active policy, prior context, or stable task defaults, and the unresolved variables concern execution detail rather than goal identity.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 4.1 Delegation vs co-creation classification
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 4.1 Delegation vs co-creation classification

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.1 Delegation vs co-creation classification

The runtime MUST choose `delegation` when:

- the objective is recoverable from active policy, prior context, or stable
  task defaults, and
- the unresolved variables concern execution detail rather than goal identity.

The runtime MUST choose `co_creation` when:

- multiple materially different objectives remain plausible, and
- silently selecting one would define the user's goal rather than execute it.

Co-creation is about unresolved objectives, not missing factual details. If the
goal is known and only a factual parameter is missing, the runtime should stay
in `delegation` and choose among `assume`, `ask_decisive`, or `block`.
