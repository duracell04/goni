---
id: "GONI-PROP-SEM-01"
title: "Proposed format-neutral semantic vocabulary for GONI"
type: "proposal"
status: "draft"
implementation_state: "specified_only"
proposition: "Investigate a minimal typed, referenceable logical vocabulary for GONI's values, claims, sources, transformations and events so that meaning and identity may be preserved across supported serializations without elevating any specific file format or programming language to universal semantics."
domains:
- "data"
- "research"
- "software"
- "system"
aliases:
- "format-neutral-semantic-core"
relations:
- type: "derived_from"
  target: "EVID-SEMANTIC-FOUNDATIONS-2026"
- type: "depends_on"
  target: "GONI-DECISION-9AF49466170D"
- type: "refines"
  target: "GONI-PRINCIPLE-9062425CD490"
sources:
- "SRC-CODD1970-DATA-INDEPENDENCE"
- "SRC-PIERCE2002-TAPL"
- "SRC-W3C2014-RDF-CONCEPTS"
- "SRC-W3C2013-PROV"
- "SRC-WICKHAM2014-TIDY"
artifacts: []
uncertainty: "An abstract logical type vocabulary is an engineering proposal, not a proven smallest or complete basis. Applicability to arbitrary scientific, multimodal, real-time and policy objects, conversion fidelity, representational cost and compatibility with D-002 remain unresolved."
legacy: []
---

# GONI-PROP-SEM-01 — Format-neutral semantic vocabulary

**Status: research proposal, specified only.** The existing D-002 Arrow canonical contract, memory classes and kernel permission model remain unchanged pending a separately governed decision.

## The problem is semantic, not syntactic

A bit, an LLM token, a JSON object and an Arrow column are units of different *abstraction levels*. Bytes can encode a value without specifying its meaning. Token boundaries are tokenizer-dependent. A semantic model needs explicit type interpretation, stable identity, data structure and applicable evidence/authority metadata.

The *smallest sufficient grammar* is a design objective, not a theorem. Use the following as a candidate logical type algebra, independently of serialization:

```text
Scalar ::= Boolean | Integer | Decimal(precision,scale)
         | Text(encoding) | Bytes | Timestamp(unit,timezone) | Identifier(namespace)
Type   ::= Scalar | Optional(Type) | Record({field:Type})
         | Variant({case:Type}) | Sequence(Type) | Map(Key,Type)
         | Reference(Type)
```

Dates, currencies, units of measure, domains of validity, confidence and security markings require *meaningful constraints* in addition to scalar storage types. Missing, null, zero and unknown must be distinguishable where relevant. Equality and ordering depend on the type, e.g. sequences are ordered and multi-sets are not.

## Five composable semantic object classes

| Object | Minimal semantics | Why it cannot be collapsed into a bare value |
| --- | --- | --- |
| Typed value | Declared type, value, domain-specific precision/units | `149` is not a currency amount without units and context. |
| Assertion | Persistent statement ID, subject, predicate, object, source references, validity and epistemic status | `price=149` might be supplier-reported, inferred, superseded or conditional. |
| Transformation description | Operation identity/version, typed signature and semantic rule or implementation reference | Describing a function in data does not execute it or prove it correct. |
| Event | Occurrence identity, actor, time, observed inputs and state transition/outcome reference | Observing a state or receiving a command is different from a verified effect. |
| Capability reference | Canonical authority object ID and applicable scope | A document referring to a permission does not grant it. |

These five classes are *application-level primitives*. Record, variant, collection and reference are the **type constructors** from which they can be built. Do not mistake the application vocabulary for a universal set of mathematically primitive atoms.

### Identity separation

Keep distinct: (i) entity identity, (ii) assertion identity, (iii) bytes/content identity (including hash), (iv) version identity, and (v) event/execution identity. Identical content hashes indicate identical bytes under the chosen hashing contract, not identical truth, owner or permitted purpose.

### Example: knowledge as a source-scoped assertion

```text
Assertion:
  id: claim-017
  subject: supplier-offer-42
  predicate: unit-price
  object: Money(amount=35.00, currency=USD)
  epistemic_kind: supplier-reported
  source: document-42 / page-3
  recorded_at: 2026-11-25T00:00:00Z  # illustrative
  valid_interval: offer-specific
```

The same typed assertion might be serialized into JSON, RDF/JSON-LD with an adequate assertion/provenance model, Arrow rows, SQL tables or a human-readable Markdown view. **Semantic equivalence is conditional** on the representational contract; a bare RDF triple, unschematized CSV or lossy export may omit essential properties.

## Canonical scope and views

- Governed source assets remain original evidence.
- Canonical facts, claim versions and authority remain governed by existing GONI memory, Data and Control contracts.
- Graph, searchable text, embeddings, Markdown and serialized exports are explicit derived projections; identical-looking statements need provenance reconciliation.
- An interchange adapter declares its supported type subset, conversions, loss policy, version, and verification evidence.
- Portable means meaningful reconstruction **for a declared subset**, rather than arbitrary binary-level interchangeability.

An early pilot can use existing files and schemas and a small compatibility test. This proposal authorizes **no new universal database, ontology server, always-on index or extra GONI architectural plane**.

## Formal evaluation targets

For supported `x : T` and format `F`:

$
D_F(E_F(x)) \equiv_T x.
$

Here `≡_T` must specify the preserved semantics: type, unit, normalization, orderedness, identity and all required provenance/visibility information. Unsupported or lossy conversions must be identifiable rather than silently asserted equivalent.

For a round trip through two formats `F` and `G`, verify declared invariants under composition, including deliberate counterexamples (missing currency, timestamp offset loss, duplicate graph edges, missing value vs null and conflicting evidence).

## Alternatives and decision boundary

**Alternative A — retain Arrow as both logical and physical canonical.** Minimal architectural change and good analytics performance, but can entrench specific physical assumptions for mutable or unstructured objects.

**Alternative B — define a small logical vocabulary above Arrow (candidate).** Greater portability and clearer semantics, at the cost of adapters, duplicated conceptual layers and verification effort.

**Alternative C — use an existing RDF ontology or generic universal IR for everything.** Mature semantic vocabularies exist, but universalization risks complexity, poor fit for dense analytics and confusing authority with knowledge.

The initial research should prefer **the smallest useful contract applied to one real workflow**, and adopt B only if the measured benefits outweigh the conversion and governance costs. D-002/D-003 replacement is not implied by this proposal.

## Related GONI contracts

The intended correspondence is with the Arrow Spine (`GONI-DECISION-655CDB324A76`), MemoryEntries, `MEM-RETR-01`, `LSC-01`, `SS-01` and kernel authority (`GONI-DECISION-9AF49466170D`). Proposed execution semantics and authority invariants live in `GONI-PROP-COMP-01`; the test belongs to `EXP-SEM-PORT-01`.
