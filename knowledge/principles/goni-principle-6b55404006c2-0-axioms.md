---
id: GONI-PRINCIPLE-6B55404006C2
title: 0. Axioms
type: principle
status: draft
implementation_state: specified_only
proposition: '| Axiom | Name | Statement | Enforcement | | SMA | Single-Model Axiom | Every persistent or transient entity is exactly one row in a canonical table.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/10-axioms-and-planes.md
  heading: 0. Axioms
  revision: 43a497b2a7deb59e07ad598a7c0496fbc9dc3cbe
---

# 0. Axioms

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 0. Axioms

| Axiom | Name | Statement | Enforcement |
|------|------|-----------|-------------|
| SMA  | Single-Model Axiom | Every persistent or transient entity is exactly one row in a canonical table. | Schema registry + `goni-schema` + Clippy lint |
| ZCO  | Zero-Copy Ontology | Cross-crate APIs expose only Arrow batches + opaque IDs (`RecordBatch`/`Arc<RecordBatch>`). | `forbid(non_arrow_entity)` lint + API review gate |
| TXT  | Text Confinement | Raw text (`LargeUtf8` > 512 bytes) exists only in `Chunks.text` and `Prompts.text`. | Schema validator + macro guard |

Violation of any axiom is a compile-time error in CI.
