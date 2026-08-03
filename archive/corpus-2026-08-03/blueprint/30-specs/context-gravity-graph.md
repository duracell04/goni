---
id: CGG-01
type: SPEC
status: specified_only
---
# Context Gravity Graph
DOC-ID: CGG-01

Status: Specified only / roadmap

The Context Gravity Graph is Goni's contract for turning memory from static
storage into task-conditioned contextual salience. Product language may call
this a "gravitational field": every prior decision, artifact, correction, and
memory can exert pull on future work. The normative contract below uses the
implementation terms: typed nodes, reasoned edges, traversal, salience, decay,
and Context Plane materialization.

This spec extends Governed Memory Retrieval (MEM-RETR-01). It does not change
the `/v1/chat/completions` API, and it does not add a shipping table to the
executable `goni-schema` DSL. Graph databases, ANN indexes, and caches may be
derived backends, but Arrow rows remain the local-first source of truth.

The Context Gravity Graph does not replace retrieval. It governs which
retrieved, remembered, and inferred materials are allowed to exert salience on
a Work Order, then compiles that influence into a `ContextPack` with receipts.

## 1. Scope

CGG-01 applies when retrieval uses graph structure to assemble Context Plane
material for an LLM call, tool-mediated action, reconstruction preview, or
delegated Work Order.

It defines:

- how existing canonical rows act as graph nodes,
- the specified-only future `KnowledgeGraphEdges` table concept,
- the specified-only `ContextPack` artifact produced for one Work Order,
- the scoring inputs for temporal salience,
- the context assembly flow from Work Order to `ContextItems`,
- the receipt metadata required to audit graph-influenced retrieval.

It does not define a concrete graph query language, graph database, embedding
model, UI visualization, or autonomous edge-mining implementation.

## 2. Nodes

Graph nodes are existing rows in canonical Arrow tables. A graph node reference
MUST identify the table kind and row ID. Eligible node kinds include, but are
not limited to:

- Knowledge rows: `Docs`, `Chunks`, `Embeddings`, `MemoryEntries`,
  `StateSnapshots`, `StateDeltas`, and `LatentSummaries`.
- Context rows: `Prompts` and `ContextItems`.
- Control rows: `Requests`, `Tasks`, `WorkOrders`, `AuditRecords`,
  `CapabilityTokens`, and `AgentManifests`.
- Execution rows when relevant as evidence or telemetry waypoints:
  `LlmCalls`, `PlatformSignals`, `PlatformCapabilities`, and `Metrics`.

Node content authority comes from the underlying table and its policy metadata.
CGG-01 MUST NOT create a second source of truth for node text, permissions, or
retention state.

## 3. ContextPack

A `ContextPack` is the compiled context bundle produced by graph traversal,
reranking, compression, and policy filtering for one Work Order.

`ContextPack` is specified only in CGG-01. It is not a shipping canonical table
or API object until a later schema/API revision promotes it. Implementations may
represent it as a replayable artifact, receipt-linked metadata, or derived
Context Plane state, but they MUST preserve this logical shape:

```yaml
context_pack_id:
work_order_id:
graph_snapshot_id:
scoring_policy_id:
decay_policy_id:
permission_filter_ref:
token_budget:
selected_context_items:
excluded_candidates:
compression_policy:
assembly_reason:
receipt_ref:
created_at:
provenance:
```

`selected_context_items` references the material selected for the prompt bundle,
usually `ContextItems` plus source waypoints. `excluded_candidates` records
bounded refs and omission reasons for high-salience or high-similarity
candidates that did not enter the pack. `assembly_reason` is a bounded summary
or hash/ref pair, not raw free-form rationale text.

## 4. Edges

`KnowledgeGraphEdges` is a specified-only future table concept. It is not part
of the shipping schema DSL until a schema revision promotes it.

An implementation that materializes graph edges MUST preserve these logical
fields, whether stored as Arrow rows, derived indexes, or replayable artifacts:

| Field | Meaning |
| --- | --- |
| `edge_id` | UUIDv7 row ID if persisted as a canonical row. |
| `source_ref` | Typed source node reference, e.g. `MemoryEntries:<uuid>`. |
| `target_ref` | Typed target node reference. |
| `edge_type` | Controlled label from the ontology below. |
| `explicit_user_weight` | Optional user-set weight in `[0, 1]`. |
| `system_inferred_weight` | Optional parser/model/system-inferred weight in `[0, 1]`. |
| `usage_reinforced_weight` | Optional reinforcement weight from repeated accepted use in `[0, 1]`. |
| `final_weight` | Derived inspectable weight before decay and policy filtering. |
| `reason_summary` | Bounded summary of why the edge exists. |
| `reason_ref` | Optional hash or source ref for replaying the rationale. |
| `scope_refs` | Project, person, Work Order, policy, or task refs where the edge applies. |
| `confidence` | Float in `[0, 1]` expressing extraction or assertion confidence. |
| `permission_scope` | Finite permission label compatible with MemoryEntries. |
| `quoteability` | Finite quoteability label when edge traversal may surface source content. |
| `valid_from`, `valid_until` | Temporal validity window. |
| `ttl_ms` | Optional expiry budget. |
| `decay_policy` | Finite label or config ref for temporal decay. |
| `conflict_state` | Finite state for normal, conflicted, superseded, quarantined, or pending-review edges. |
| `provenance` | Parser, model, user action, receipt, policy, and source refs. |

Raw edge rationale text MUST NOT be stored in edge rows by default. Long text
belongs only in permitted Knowledge or Context Plane fields, currently
`Chunks.text` and `Prompts.text`. Edge rows use summaries, hashes, and refs.

### 4.1 Edge Ontology

Edge types are controlled because they affect salience differently. The minimum
ontology is:

| Edge type | Salience semantics |
| --- | --- |
| `supports` | Positive evidence for a claim, decision, or context candidate. |
| `contradicts` | Negative or competing evidence that SHOULD be retrievable and surfaced as uncertainty, not silently ignored. |
| `supersedes` | Shifts authority toward the source or target marked as newer by provenance and validity metadata. |
| `refines` | Narrows or improves a prior node without fully replacing it. |
| `depends_on` | Pulls prerequisite context when the current Work Order needs the dependent node. |
| `inspired_by` | Weak creative or conceptual affinity; useful for ideation, lower weight for factual tasks. |
| `same_theme_as` | Cluster-level thematic similarity; useful for recall expansion but weaker than evidence edges. |
| `applies_to` | Scopes a memory, policy, decision, or skill to a project, person, task, or Work Order. |
| `blocks` | Indicates a constraint or unresolved issue that can prevent use or require surfacing. |
| `derived_from` | Provenance edge from source material to derived memory, summary, decision, or artifact. |

Implementations MAY add labels only through a schema/spec revision or controlled
configuration with stable semantics. `supports` and `contradicts` may both
increase retrieval priority, but they MUST NOT be treated as equivalent during
reranking or prompt assembly.

### 4.2 Weight Sources

Graph weight has three inspectable sources:

```yaml
weight:
  explicit: 0.9
  inferred: 0.62
  reinforced: 0.74
  final: 0.83
```

`explicit` is set by the principal or an authorized user action. `inferred` is
created by parsers, models, importers, or deterministic rules. `reinforced` is
updated from accepted repeated use, citations, corrections, or confirmations.
`final` is derived from these components by the scoring policy and MUST remain
auditable. A user-set explicit weight MAY dominate inferred and reinforced
weights when policy allows.

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

### 5.1 Conflict-preserving query modes

Contradiction does not grant the graph authority to manufacture consensus. A
Work Order that encounters materially conflicting claims MUST preserve both
sides and select behavior according to the query purpose:

| Query mode | Required behavior |
| --- | --- |
| `descriptive` | Return the material competing claims, provenance, confidence, validity, and conflict basis. |
| `historical` | Apply the requested validity window and distinguish later amendments or supersession. |
| `operational` | Apply a controlling rule only when the principal or an explicitly delegated role supplied one; retain material dissent as context. |

If an operational query has no authorized controlling rule, the runtime MUST
surface the conflict, ask, or escalate under the Work Order rather than infer
authority from graph weight, model confidence, source count, or majority
agreement.

A conflict resolution may change which node controls future operational
selection, but MUST NOT delete or rewrite the competing node, its provenance,
or the `contradicts` relationship. The decision and its authority basis MUST be
receipt-linked. Formal policy and observed practice remain separate nodes or
claims even when one controls the current operation.

## 6. Identity Resolution

CGG-01 distinguishes identity from relation. Similar phrases may describe the
same concept, aliases for one object, or merely related ideas.

Graph implementations SHOULD preserve:

- canonical IDs for durable concepts, projects, people, artifacts, and memory
  entries,
- aliases and surface forms,
- duplicate-detection evidence,
- concept cluster refs,
- merge/split provenance and undo refs.

Merge operations MUST preserve source refs and receipts. Split operations MUST
preserve prior aliases and explain why one cluster became multiple concepts.
Concept clusters may influence salience through `same_theme_as`, `refines`, or
`applies_to` edges, but they do not override canonical node identity.

The ontology MUST be no broader than necessary for retrieval, permission,
temporal reasoning, and principal-directed action. Merge and split operations
MUST preserve dissent, rationale, prior identities, and undo refs. Inferred
relationships MUST remain distinguishable from principal-set or imported
explicit relationships. No merge, cluster score, or canonical label creates
truth or operational authority by itself: the map is not the territory.

## 7. Context Compression Policy

Context assembly often finds more relevant material than fits in the prompt
window. A ContextPack MUST record the compression policy used for each selected
item when compression affects the prompt bundle.

Allowed compression forms:

| Form | Use |
| --- | --- |
| `raw_excerpt` | Source-grounded tasks that need exact wording or citations. |
| `summary` | General tasks where bounded prose is sufficient. |
| `latent_summary` | Compact state or derived memory where raw text should not be sent. |
| `decision_only` | Tasks that need the resulting decision or rule, not the full discussion. |
| `citation_only` | Tasks that need a waypoint/reference but not content in the model context. |

The Work Order type, risk class, output shape, permission scope, quoteability,
and token budget SHOULD drive compression choice. A legal memo may prefer
`raw_excerpt`; a style-sensitive social draft may prefer `summary` or
`latent_summary`; a high-risk action may include `citation_only` refs for
audit while withholding sensitive content from the model.

## 8. Context Assembly Flow

Graph-influenced context assembly MUST preserve the MEM-RETR-01 pipeline and
add graph traversal as a bounded retrieval stage:

1. Bind retrieval to a `WorkOrder`; if no Work Order exists, create one or
   record why a read-only lookup is allowed without it.
2. Generate seed candidates from dense, sparse, exact-match, metadata, and
   policy-bounded lookups.
3. Expand from seeds through permitted graph edges under configured depth,
   fanout, token, and latency budgets.
4. Score candidates with the salience function.
5. Filter by permission, quoteability, validity, conflict state, source trust,
   and parser confidence.
6. Rerank candidates and run existing submodular selection under token budget.
7. Choose compression forms for selected candidates.
8. Compile a `ContextPack` for the Work Order.
9. Materialize only selected evidence into the Context Plane as `ContextItems`
   or prompt material derived from selected refs.
10. Cite selected evidence with source waypoints sufficient for audit.
11. Emit receipts for memory reads, graph retrieval basis, omissions,
    compression choices, and context materialization when retrieval affects
    output or execution.

Graph traversal is not a bypass around policy. It is one retrieval signal among
dense, sparse, exact-match, metadata, and reranking signals.

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

## 10. Safety Invariants

- Graph edges MUST NOT turn untrusted source text into Control Plane
  instruction without policy mediation.
- Graph traversal MUST NOT increase the authority of observed screen, browser,
  OCR, audio, or accessibility-derived material beyond its memory grant.
- Private or relationship-scoped edges MUST NOT cause remote context disclosure
  unless policy explicitly allows the destination and purpose.
- Deletion, redaction, tombstoning, or permission revocation MUST remove or
  demote affected nodes and edges from normal traversal.
- Edge extraction confidence MUST remain visible to scoring and receipts.
- Graph-derived uncertainty SHOULD be surfaced when conflicting edges materially
  affect selected context.
- User-specified weights MUST remain inspectable and MUST NOT be silently
  overridden by inferred or reinforced weights.

## 11. Upstream

- [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md)
- [Latent state contract](/blueprint/30-specs/latent-state-contract.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Vision, memory, and actuation boundaries](/blueprint/30-specs/vision-memory-actuation-boundaries.md)
- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)

## 12. Downstream

- [Retrieval component](/blueprint/software/retrieval/README.md)
- [Vector database](/blueprint/software/30-components/vecdb.md)
- Future schema revision for `KnowledgeGraphEdges` and optional `ContextPacks`
- [Local Sovereign Knowledge Runtime](/blueprint/20-system/65-local-sovereign-knowledge-runtime.md)

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
