# LSC-01 - Latent State Contract
DOC-ID: LSC-01
Status: Specified only / roadmap
Conformance: TBD (goni-lab harness)

This document defines the kernel-owned latent state contract for Goni OS. It is
model-agnostic and describes the minimal records, interfaces, and invariants
required for latent-first cognition.

SS-01 defines the symbolic substrate and arbitration contract used when validating state updates and tool calls.

## 1. Purpose

The Latent State Store (LSS) is the kernel primitive that maintains "what is
going on" without requiring continuous LLM decoding. It is the canonical source
for:

- `S_core`: small dense working state (hot, always resident).
- `Delta` stream: append-only updates for reconstruction.
- `F_sparse`: keyed facts/flags (typed, symbolic).

All state changes are policy-checked and auditable.

## 2. Canonical records (data plane)

The LSS uses these canonical tables (see `blueprint/software/50-data/51-schemas-mvp.md`
and `blueprint/software/50-data/53-schema-dsl-and-macros.md`):

- `StateSnapshots`: point-in-time snapshots of `S_core` + `F_sparse`.
- `StateDeltas`: append-only deltas applied to a snapshot.
- `LatentSummaries`: compact derived summaries (optional, budgeted).

These records are immutable once written.

## 3. Interfaces (kernel API)

Minimal kernel APIs (names are illustrative):

- `read_state(snapshot_id) -> StateSnapshot`
- `append_delta(delta: StateDelta) -> delta_id`
- `checkpoint(snapshot: StateSnapshot) -> snapshot_id`
- `summarize(range, policy) -> LatentSummary`

All interfaces are capability-mediated and produce audit records.

## 4. Invariants

- **Append-only deltas:** `StateDeltas` are never modified or deleted in place.
- **Crash consistency:** state is reconstructable from the latest snapshot plus
  ordered deltas within the retention window.
- **Policy mediation:** every write is validated by the policy engine.
- **Latent-first loop:** steady-state updates do not require LLM decoding.
- **Provenance attached:** each record includes `provenance` metadata.

## 4.1 Memory write policy (normative)

MemoryEntries are governed by explicit write gates:
- `kind` MUST be one of: `fact`, `preference`, `decision`, `hypothesis`, `derived`.
- `fact` MUST include at least one `source_chunk_id` or a `confirmed_by_event_id`.
- `hypothesis` MUST set `ttl_ms` or `review_at` and MUST NOT be promoted to `fact`
  without a confirmation event or new source evidence.
- `derived` MUST include source references and provenance for the transform.

Writes that do not meet these requirements MUST be rejected by policy checks.

## 5. Audit requirements

All LSS writes MUST include the following audit fields (directly or by
reference):

- `agent_id`
- `policy_hash`
- `state_snapshot_id`
- `provenance`

See `blueprint/docs/specs/tool-capability-api.md` for the audit record envelope.

## 6. Provenance format (minimal)

`provenance` is a structured object that includes:

- `source`: origin (observer, encoder, tool, agent).
- `timestamp`: event time (UTC).
- `inputs`: references to upstream record IDs.
- `permissions`: policy tags in effect.

## 7. Related specs

- [agent-definition.md](/blueprint/docs/specs/agent-definition.md)
- [agent-manifest.md](/blueprint/docs/specs/agent-manifest.md)
- [tool-capability-api.md](/blueprint/docs/specs/tool-capability-api.md)
- [scheduler-and-interrupts.md](/blueprint/docs/specs/scheduler-and-interrupts.md)
- [symbolic-substrate.md](/blueprint/docs/specs/symbolic-substrate.md)

## 8. Upstream
- [Schema MVP](/blueprint/software/50-data/51-schemas-mvp.md)
- [Schema DSL and macros](/blueprint/software/50-data/53-schema-dsl-and-macros.md)
- [Symbolic substrate](/blueprint/docs/specs/symbolic-substrate.md)

## 9. Downstream
- [Agent definition](/blueprint/docs/specs/agent-definition.md)
- [Tool capability API](/blueprint/docs/specs/tool-capability-api.md)
- [Latent predictor](/blueprint/software/30-components/latent-predictor.md)

## 10. Adjacent
- [Scheduler and interrupts](/blueprint/docs/specs/scheduler-and-interrupts.md)
- [System map](/blueprint/docs/00-system-map.md)

## Conformance tests
- TBD: add tests for this spec.




