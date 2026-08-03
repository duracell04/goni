---
id: GONI-SPEC-CD664BF8E016
title: Acceptance Fixtures
type: specification
status: draft
implementation_state: specified_only
proposition: 'Cross-project idea: one concept touches three projects; traversal includes only the project/person scopes allowed by the Work Order.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: Acceptance Fixtures
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# Acceptance Fixtures

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Acceptance Fixtures

- Cross-project idea: one concept touches three projects; traversal includes
  only the project/person scopes allowed by the Work Order.
- Superseded memory: an old decision is linked to a newer decision with
  `supersedes`; the old decision is demoted unless history is requested.
- Policy-denied edge: a high-similarity private memory is omitted with
  `permission_denied` and does not enter selected context.
- Old pinned foundational node: an old thesis remains salient despite decay
  because it is pinned, while still obeying policy filters.
- Contradictory nodes: two candidate facts linked by `contradicts` are surfaced
  as uncertainty rather than merged into one assertion.
- Formal versus practice: a formal rule and conflicting observed practice both
  remain retrievable; only an explicit authority rule controls an operational
  decision.
- Unauthorized resolution: a high-confidence graph majority cannot resolve a
  contradiction or create operational authority without a principal or
  delegated rule.
- Reversible ontology: a concept merge preserves both prior IDs, dissent, and
  an undo ref; reversing it restores the prior identities.
- Deterministic ordering: the same Work Order, graph snapshot, scoring policy,
  decay policy, token budget, and indexes produce the same ContextPack ordering.
