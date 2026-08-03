---
id: GONI-SPEC-4DACDEB99E62
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: A retrieval-mediated action emits memory_read_refs.
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
  heading: Conformance tests
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests

- A retrieval-mediated action emits `memory_read_refs`.
- A memory mutation emits `memory_diff_refs`.
- A parser-mediated memory write emits `parser_basis`.
- A graph-mediated retrieval emits graph snapshot/config refs, scoring policy,
  decay policy, ContextPack refs, selected refs, omission reasons, compression
  policy, and permission filters in `retrieval_basis`.
- A correction-derived memory write emits a learning receipt and
  `memory_diff_refs`.
- Retrieval against the same Work Order, fixed index, fixed reranker, and fixed
  policy hash is deterministic.
- Expired or policy-denied memory is absent from selected context.
- Raw retrieved text is confined to allowed Knowledge/Context Plane fields.
- Raw parser output is confined to allowed Knowledge/Context Plane fields.
- desktop/browser/vision-derived memory writes require a memory grant separate
  from observation and extraction permission
- A technical representation, machine enrichment, human interpretation, or
  principal assertion retains its derivation stage and original source refs.
- A derived artifact cannot overwrite or impersonate original evidence.
- A model-produced claim cannot become a principal assertion or controlling
  operational rule without an authorized event and receipt.
- An as-of query selects evidence by the requested validity window instead of
  silently applying the current version.
- Conflicting formal policy and observed practice remain separately
  attributable and retrievable.
