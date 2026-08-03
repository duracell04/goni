---
id: GONI-SPEC-ECC06274174E
title: 9. Receipt Contract
type: specification
status: draft
implementation_state: specified_only
proposition: 'When graph traversal affects output, execution, or reconstruction previews, retrieval_basis SHOULD include: retrieval mode: graph or mixed, ContextPack ref, graph snapshot or index version, traversal depth and fanout limits, scoring policy ID or hash, decay policy ID or hash, selected node refs and selected edge refs, inclusion reasons for selected context,'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 9. Receipt Contract
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 9. Receipt Contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. Receipt Contract

When graph traversal affects output, execution, or reconstruction previews,
`retrieval_basis` SHOULD include:

- retrieval mode: `graph` or `mixed`,
- ContextPack ref,
- graph snapshot or index version,
- traversal depth and fanout limits,
- scoring policy ID or hash,
- decay policy ID or hash,
- selected node refs and selected edge refs,
- inclusion reasons for selected context,
- omission reasons for high-similarity or high-salience candidates,
- excluded candidate counts by policy, expiry, conflict, permission, token
  budget, compression loss, supersession, or weak edge reason,
- compression policy and selected compression forms,
- permission and quoteability filters,
- policy hash,
- Work Order ref.

Receipts MUST NOT store raw source text or raw edge rationale by default.
Receipts store refs, hashes, bounded summaries, and replay metadata.

An omission reason answers why an apparently relevant candidate did not enter
the ContextPack. Valid omission reasons include `stale`, `over_token_budget`,
`permission_denied`, `quoteability_denied`, `edge_too_weak`, `superseded`,
`contradicted_unresolved`, `quarantined`, `archived`, `duplicate_clustered`,
and `compression_not_allowed`.
