---
id: GONI-SPEC-341F43600D0F
title: 2.1 Source and derived-artifact separation
type: specification
status: draft
implementation_state: specified_only
proposition: The retrieval pipeline MUST preserve the distinction between original evidence and every representation or interpretation derived from it.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/memory-retrieval.md
  heading: 2.1 Source and derived-artifact separation
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# 2.1 Source and derived-artifact separation

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Source and derived-artifact separation

The retrieval pipeline MUST preserve the distinction between original evidence
and every representation or interpretation derived from it. A source, technical
representation, machine enrichment, human interpretation, and principal or
delegate assertion are different artifacts even when they describe the same
subject.

Implementations MUST preserve a bounded derivation stage in existing
`MemoryEntries.value` or `MemoryEntries.provenance`, or through stable refs to
the corresponding source and receipt. The logical stages are:

`technical_representation | machine_enrichment | human_interpretation |
principal_assertion`.

These labels do not add a schema field or expand the finite `MemoryEntries.kind`
enum. Derived memory continues to use existing kinds, source refs, and
provenance maps.

- Derived artifacts MUST retain source refs, parser or model identity where
  applicable, confidence, permission scope, validity, and receipt refs.
- Derived artifacts MUST NOT overwrite or impersonate original evidence.
- If authorized retention, deletion, or redaction removes a source, dependent
  artifacts MUST expose the source as unavailable, redacted, or tombstoned.
- Model output MUST NOT promote itself into a principal assertion, policy, or
  controlling operational rule. That requires an explicit principal action or
  an authorized delegated event with a receipt.
- Formal policy and observed practice MUST remain separately attributable when
  they conflict. Neither silently rewrites the other.

Retrieval MUST preserve four temporal meanings when they are material:

1. when the source event occurred,
2. when the system recorded or derived the artifact,
3. the `valid_from` / `valid_until` window in which the claim or rule applied,
4. when it was superseded, withdrawn, redacted, or otherwise ceased to control.

Existing timestamps, validity fields, conflict state, provenance maps, CGG
edges, and receipt refs carry these meanings; this contract introduces no new
canonical table. As-of queries MUST apply the requested validity window and
MUST NOT silently substitute a current claim for a historical one.
