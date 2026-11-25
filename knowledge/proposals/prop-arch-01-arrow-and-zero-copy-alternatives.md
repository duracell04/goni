---
id: "GONI-PROP-ARCH-01"
title: "Proposed clarification of D-002 Arrow canonicality and D-003 zero-copy hot paths"
type: "proposal"
status: "draft"
implementation_state: "specified_only"
proposition: "Assess whether GONI should distinguish its abstract logical type and transformation contracts from Arrow's physical RecordBatch implementation, while restricting affine zero-copy obligations to declared view/transport operations rather than every useful computational hot-path transform."
domains:
- "data"
- "research"
- "software"
- "system"
aliases:
- "d002-d003-semantic-physical-separation"
relations:
- type: "derived_from"
  target: "GONI-DECISION-655CDB324A76"
- type: "derived_from"
  target: "GONI-DECISION-E18EFE88D5EB"
- type: "depends_on"
  target: "GONI-PROP-SEM-01"
- type: "depends_on"
  target: "GONI-PROP-COMP-01"
sources:
- "SRC-APACHE-ARROW-COLUMNAR"
- "SRC-CODD1970-DATA-INDEPENDENCE"
- "SRC-PARNAS1972-INFORMATION-HIDING"
- "SRC-LLVM-MLIR-IR"
artifacts: []
uncertainty: "A revision of D-002/D-003 would be consequential architectural governance; it is not approved by this proposal. Existing affine and zero-copy closure claims depend on precise buffer/aliasing semantics and verification, while a separate logical layer may create unnecessary cost and complexity."
legacy: []
---

# GONI-PROP-ARCH-01 — D-002/D-003 refinement options

**Status: proposed architectural clarification only.** The decision records `D-002` (`GONI-DECISION-655CDB324A76`) and `D-003` (`GONI-DECISION-E18EFE88D5EB`) remain as authored. This file proposes a review, not a change to their acceptance status or production implementation.

## Present specification and conceptual tension

**D-002** specifies the category `A` through Arrow schemas and RecordBatches and requires all structured state to be expressible using that category. This creates one interoperable, efficient data representation, but also entangles canonical *meaning* with a particular *physical data layout*. Arrow is itself language independent; the concern is physical-layout dependence, **not** that Arrow mandates one programming language.

**D-003** declares affine, zero-copy transforms on the entire defined hot path. Its premise fits metadata-only slicing and transport well. However, deriving a new value by addition, aggregation, decoding or inference usually needs computation and output storage. Requiring no new payload allocation for every useful per-request transformation would constrain the architecture beyond what the workload justifies.

**Academic basis:** [Codd (1970)](https://doi.org/10.1145/362384.362685) motivates data independence; [Parnas (1972)](https://doi.org/10.1145/361598.361623) motivates preserving replaceable implementation choices; [Apache Arrow](https://arrow.apache.org/docs/format/Columnar.html) documents the hardware benefits and mutation trade-off of columnar layout. These precedents motivate an investigation; they do not prove GONI should change either decision.

## Considered candidate interpretations

| Option | Logical authority | Physical runtime | Benefit | Cost or risk |
| --- | --- | --- | --- | --- |
| A. Preserve D-002/D-003 exactly | Arrow schema/category | Arrow RecordBatch and affine view operations | Existing conceptual consistency; high-performance analytics | May force awkward representations and exclude new allocating computations from `hot`. |
| B. Separate logical semantics from physical placement (candidate) | Typed values, claims, operations and versioned authority contracts | Arrow preferred for suitable structured workloads; other governed source/object/transactional stores where justified | Smaller interoperable meaning contract; more flexible storage and engines | Extra adapter surface, ambiguity if both logical and physical state are called “canonical”, and verification burden. |
| C. Generic universal IR/knowledge graph for all domains | Universal schema/IR | Backends under a large translation layer | Uniform interface ambition | High ontology/compiler complexity, semantic impedance, possible cost and lower system reliability. |

The decision should be based on matched task outcomes rather than aesthetic preference for a formal category.

## Precise clarification candidates if B earns adoption

1. **Logical contract:** define types, values, assertions, transformations, events and required identity/provenance/visibility semantics independently of physical layout. “Canonical” identifies the authoritative *logical object and owning state*, not that every datum must be duplicated into a separate semantic database.
2. **Arrow placement:** keep Arrow as a first-class, zero-copy-capable representation for validated columnar workloads. Support governed adapters with typed, versioned loss/precision policies; preserve other existing canonical authority and source objects.
3. **Transform classification:** partition operations by semantics and memory obligations:
   - **View/transport:** metadata-only slice/projection where the given buffer and aliasing contract makes zero-copy feasible;
   - **Calculating:** derived numerical or structured results, with explicit allocation/latency/resource budgets;
   - **Probabilistic:** model execution, calibration and quality budgets;
   - **Effectful:** capability mediation, transaction semantics and receipts.
4. **“Hot” is a workload property**, not a synonym for “zero-copy”; per-request computations can be latency-critical even when they necessarily allocate output.
5. **Formal composition:** specify identity and composition laws for the selected pure-transform subset. Verify whether the proposed affine ownership constraints are closed under each implemented combinator, including slices, shared buffers, branches, joins and lifetimes. A claim that abstract composition is zero-copy does not automatically establish that a concrete implementation performs zero allocations.
6. **Authority:** a format adapter or computation backend never becomes authoritative for mandate, policy, approval or revocation merely because it carries those fields.

## Falsification conditions

Option B is disfavored if representative tasks require costly lossless adapters, added schema duplication or additional storage governance without improving portability, correctness or lifecycle economics. It is also disfavored if an Arrow-native protocol already solves the same requirements with fewer mechanisms.

The proposal must distinguish a *logical schema* from a *separate runtime service*. A documented interface and a small set of tests may suffice. `EXP-SEM-PORT-01` supplies candidate gates and negative cases.

## Explicit non-decisions

No D-002 or D-003 text is superseded. No new canonical database, generalized category interpreter, mandatory RDF store, MLIR compiler or new GONI plane is approved. Any eventual change of the existing normative decisions requires a separately reviewable architectural decision and evidence from representative benchmarks.
