---
id: GONI-IMAP-86E6ECED338A
title: MemoryEntries
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'PK: memory_id = row_id Fields: kind: dict<uint8, utf8> (fact|preference|decision|hypothesis|derived), memory_class: dict<uint8, utf8> (episodic|semantic|procedural|relational|project|policy), timestamp: timestamp(ms), value: map<utf8, utf8>, confidence: float32, source_chunk_ids: list<utf8>, project_refs: list<utf8>, person_refs: list<utf8>, permission_scope: dict<uint8, utf8>,'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/51-schemas-mvp.md
  heading: MemoryEntries
  revision: bb1e07945b27222152c5ea9eb3f54c46bea197fc
---

# MemoryEntries

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### MemoryEntries
- PK: `memory_id = row_id`
- Fields: `kind: dict<uint8, utf8> (fact|preference|decision|hypothesis|derived)`, `memory_class: dict<uint8, utf8> (episodic|semantic|procedural|relational|project|policy)`, `timestamp: timestamp(ms)`,
  `value: map<utf8, utf8>`, `confidence: float32`, `source_chunk_ids: list<utf8>`,
  `project_refs: list<utf8>`, `person_refs: list<utf8>`, `permission_scope: dict<uint8, utf8>`,
  `quoteability: dict<uint8, utf8>`, `valid_from?: timestamp(ms)`, `valid_until?: timestamp(ms)`,
  `provenance: map<utf8, utf8>`, `confirmed_by_event_id?: fixed_size_binary[16]`,
  `review_at?: timestamp(ms)`, `ttl_ms?: uint32`, `conflict_state?: dict<uint8, utf8>`,
  `embedding?: fixed_size_list<float32>[1536]`, `embedding_dim?: uint16`
- Notes: `source_chunk_ids` values are UUIDv7 strings referencing `Chunks.chunk_id`; raw text is not stored here.

Correction-derived preference learning uses this table without adding a new
canonical table in v1. Learned preferences from the Correction Delta Compiler
use `kind = hypothesis | preference | derived` and `memory_class =
procedural | policy | project | relational` depending on scope. The `value` and
`provenance` maps must preserve, at minimum, scope, confidence, evidence count,
contradiction count, decay policy, review status, source refs, learning receipt
refs, memory diff refs, and regression test refs. Raw correction text remains in
allowed Knowledge or Context plane fields only; memory entries store refs,
hashes, summaries, and governance metadata.

Policy-level memory class aliases map onto the finite `memory_class` enum
until a schema version adds first-class values:

| Policy alias | Canonical class | Required policy meaning |
| --- | --- | --- |
| `personal_private` | `semantic` or `episodic` | Owner-private memory; remote use denied unless policy explicitly allows distilled context. |
| `project_shared` | `project` | Project-scoped memory with explicit collaborators or workspace boundary. |
| `relationship` | `relational` | Person/counterparty memory; higher sensitivity and citation discipline. |
| `model_system` | `policy` or `procedural` | Model/runtime/system behavior memory; never user-personal by default. |
| `ephemeral` | `episodic` | Short-lived working memory with expiry or session-bound TTL. |
| `quarantine` | `policy` | Untrusted, conflicted, poisoned, or pending-review memory; excluded from normal retrieval. |

Any memory write using one of these aliases must preserve owner, class,
provenance, permission set, expiry policy, and receipt reference in
`value`/`provenance` or by stable refs. The alias is policy data, not a reason
to bypass the canonical schema.

Visual memory uses VisualAssets and VisualAssetDerivations for governed asset
metadata and lineage. Long-lived user preferences about visual style or approved
brand decisions may still be stored as MemoryEntries, but must reference
VisualAssets, receipt refs, and policy scope rather than copying raw image
content into memory values.
