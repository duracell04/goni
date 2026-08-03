---
id: GONI-SPEC-12DB951C4154
title: 5. Salience, Conflict, And Decay
type: specification
status: draft
implementation_state: specified_only
proposition: Graph traversal produces a salience score for candidate nodes and edges.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 5. Salience, Conflict, And Decay
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 5. Salience, Conflict, And Decay

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Salience, Conflict, And Decay

Graph traversal produces a salience score for candidate nodes and edges. The
score is an implementation-defined deterministic function over these inputs:

- semantic similarity to the Work Order and current request,
- graph proximity from seed candidates,
- edge type and inspectable weight components,
- recency and temporal validity,
- confidence,
- Work Order fit, including goal, done contract, task class, output shape, and
  project/person constraints,
- permission and quoteability fit,
- source trust,
- conflict state,
- decay policy.

For a fixed Work Order, policy hash, graph snapshot, index versions, and scoring
configuration, traversal and ranking MUST be deterministic. Stable row IDs MUST
be used as tiebreakers.

Expired, policy-denied, quarantined, or deleted nodes and edges MUST be
excluded from normal traversal. Stale, conflicted, superseded, low-confidence,
or out-of-scope nodes and edges MUST be demoted or surfaced as uncertainty.

Supersession is graph-aware: a `supersedes` or `evolved_from` edge SHOULD shift
salience toward the newer or more authoritative node unless policy, pinning, or
explicit Work Order scope says otherwise.

Conflict and lifecycle states have explicit behavior:

| State | Traversal behavior |
| --- | --- |
| `contradicted` | Retrieve both sides when material to the Work Order; surface uncertainty or ask/verify before relying on either side. |
| `superseded` | Demote by default and prefer the superseding node; include only when history, audit, or comparison is needed. |
| `deprecated` | Demote strongly; include only with explicit Work Order fit or citation need. |
| `archived` | Exclude from normal interactive context unless search scope includes archives. |
| `quarantined` | Exclude from normal traversal; may appear only in safety/audit workflows. |
| `policy_denied` | Exclude from context materialization and record omission reason. |
| `pinned` | Resist temporal decay and demotion, but still obey permission, quoteability, and policy filters. |

Examples:

- If node A supersedes node B, node B loses salience unless the Work Order asks
  for history or comparison.
- If node C contradicts node D, both may be retrieved, but the ContextPack must
  preserve uncertainty instead of collapsing the conflict into one fact.
- If node E is relevant but policy-denied, it cannot enter selected context and
  must appear only as a bounded omission reason where allowed.
- If node F is old but pinned, temporal decay is reduced or disabled, while
  policy and permission checks still apply.
