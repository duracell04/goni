---
id: "EVID-SEMANTIC-FOUNDATIONS-2026"
title: "Foundations for semantic independence across data and computation representations"
type: "evidence"
status: "draft"
implementation_state: "not_applicable"
proposition: "Relational data independence, type systems, RDF abstract syntax, provenance standards, Arrow's columnar format, typed intermediate representations and component interfaces provide distinct precedents for separating meaning and contracts from particular storage layouts and programming languages; they do not jointly establish an already validated universal semantic runtime."
domains:
- "data"
- "research"
- "software"
- "system"
aliases: []
relations:
- type: "supports"
  target: "GONI-PROP-SEM-01"
- type: "supports"
  target: "GONI-PROP-COMP-01"
sources:
- "SRC-CODD1970-DATA-INDEPENDENCE"
- "SRC-PARNAS1972-INFORMATION-HIDING"
- "SRC-PIERCE2002-TAPL"
- "SRC-WICKHAM2014-TIDY"
- "SRC-W3C2014-RDF-CONCEPTS"
- "SRC-W3C2013-PROV"
- "SRC-APACHE-ARROW-COLUMNAR"
- "SRC-LLVM-MLIR-IR"
- "SRC-WASM-WIT-INTERFACES"
artifacts: []
uncertainty: "These sources establish separate abstractions, not equivalence across arbitrary formats, execution engines or authority mechanisms. GONI's proposed combination has not been implemented or measured and requires explicit semantic-preservation and cost tests."
legacy: []
---

# Foundations for format-independent semantic and computational interfaces

## What the primary sources substantiate

| Source and kind | Established contribution | What it does **not** establish |
| --- | --- | --- |
| [Codd (1970), Communications of the ACM](https://doi.org/10.1145/362384.362685), peer-reviewed | Logical data modeling can shield applications from changes in internal representation. | A universal data model for every media type or behavior; physical storage independence at zero cost. |
| [Parnas (1972), Communications of the ACM](https://doi.org/10.1145/361598.361623), peer-reviewed | Module boundaries can hide design decisions likely to change. | That more layers, abstraction or generalization always improve quality or economics. |
| [Pierce (2002), MIT Press](https://mitpress.mit.edu/9780262162098/types-and-programming-languages/), textbook | A rigorous framework for types, expressions and programming-language semantics. | That a chosen finite set of GONI primitives is mathematically minimal or universally sufficient. |
| [Wickham (2014), Journal of Statistical Software](https://www.jstatsoft.org/article/view/v059i10/), peer-reviewed | Consistent tabular structure improves analytical manipulation and reusable tooling. | That every knowledge claim should be forced into a table. |
| [RDF 1.1 Concepts (W3C, 2014)](https://www.w3.org/TR/rdf11-concepts/), Recommendation | An abstract triple/graph model can be expressed through multiple concrete serializations. | Provenance completeness, temporal truth, authorization, confidence or arbitrary computation semantics. |
| [PROV-DM (W3C, 2013)](https://www.w3.org/TR/prov-dm/), Recommendation | Distinguishes entities, activities, agents and derivations for provenance descriptions. | That recorded derivation implies scientific truth or that provenance implements access controls. |
| [Apache Arrow columnar specification](https://arrow.apache.org/docs/format/Columnar.html), technical standard | Language-independent physical columnar layout for efficient analytics and sharing; zero-copy opportunities. | Ideal mutation/write performance for every workload, or intrinsic application-level semantics beyond the declared fields/types. |
| [MLIR language reference](https://mlir.llvm.org/docs/LangRef/), compiler specification | A typed operation/value IR supports multiple readable, in-memory and serialized representations. | Engine equivalence without operation semantics, verified lowerings, and backend-specific constraints. |
| [WebAssembly WIT](https://component-model.bytecodealliance.org/design/wit.html), interface specification | Interface types and functions can be declared independent of the guest programming language. | Execution behavior, effects, permissions or end-to-end correctness from interface syntax alone. |

## Evidence-to-design argument

The sources support a layered *research hypothesis*: users and cooperating agents benefit when application meaning is defined through explicit identities, types, relations, and transformation contracts, while storage layouts and execution mechanisms remain replaceable subject to compatibility tests. **No cited source alone proves the proposed GONI architecture.**

The word *primitive* is used at different levels. A bit is a binary distinction at the encoding level; a tokenizer token depends on a particular model vocabulary; a typed datum is a semantic value within a chosen model; a claim, event or transformation is a composite semantic construct. No single representation supplies the smallest unique atom of all knowledge.

Source facts and derived claims remain distinct. GONI's canonical evidence and authority state are governed by existing contracts, not inferred from the mere existence of a graph edge, document or computation.

## Status

This is externally grounded research evidence with explicit limitations, not an accepted D-002/D-003 replacement or an empirical GONI conformance result. The proposed minimal semantic grammar is in `GONI-PROP-SEM-01`; computational meaning and effects are in `GONI-PROP-COMP-01`; the comparison and tests are proposed separately.
