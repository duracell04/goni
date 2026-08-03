---
id: GONI-SPEC-D3F659B69352
title: Conformance Tests
type: specification
status: draft
implementation_state: specified_only
proposition: The spec is listed in the specs index and registry.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: Conformance Tests
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# Conformance Tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance Tests

- The spec is listed in the specs index and registry.
- No new raw text field is introduced outside `Chunks.text` and `Prompts.text`.
- A graph-influenced retrieval compiles a ContextPack for one Work Order with a
  fixed graph snapshot, scoring policy, decay policy, permission filter, and
  token budget.
- Retrieval against the same Work Order, graph snapshot, scoring config, policy
  hash, and fixed indexes returns deterministic context ordering.
- Expired, deleted, quarantined, or policy-denied edges cannot cause selected
  context.
- A superseded memory loses salience unless explicitly pinned, reinforced, or
  selected by Work Order scope.
- Graph-influenced retrieval emits receipts that explain selected context with
  graph snapshot, scoring policy, decay policy, selected refs, and filters.
- Graph-influenced retrieval emits omission reasons for high-similarity or
  high-salience candidates excluded from the ContextPack.
- Weight components remain inspectable: explicit, inferred, reinforced, and
  final.
- Compression form is recorded for every compressed selected item.
- Descriptive and historical conflict queries preserve material competing
  claims instead of collapsing them into one answer.
- An operational conflict query cannot select a controlling rule without an
  explicit principal or delegated authority basis.
- Conflict resolution changes operational selection without deleting the
  competing claim or contradiction edge.
- Ontology merges and splits preserve prior identities, dissent, rationale,
  provenance, and undo refs.
