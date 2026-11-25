---
id: "EXP-SEM-PORT-01"
title: "Semantic portability and cross-engine computation evaluation"
type: "experiment"
status: "draft"
implementation_state: "not_applicable"
proposition: "Test whether a bounded GONI supplier-offer analytical pipeline preserves declared semantics, source provenance, deterministic outputs and kernel authorization across supported storage serializations and independently implemented calculation engines, while measuring the overhead of an abstract semantic contract against an Arrow-native baseline."
domains:
- "data"
- "kernel"
- "research"
- "software"
- "system"
aliases:
- "semantic-portability-evaluation"
relations:
- type: "tests"
  target: "GONI-PROP-SEM-01"
- type: "tests"
  target: "GONI-PROP-COMP-01"
- type: "tests"
  target: "GONI-PROP-ARCH-01"
- type: "derived_from"
  target: "EVID-SEMANTIC-FOUNDATIONS-2026"
sources:
- "SRC-CODD1970-DATA-INDEPENDENCE"
- "SRC-PIERCE2002-TAPL"
- "SRC-W3C2014-RDF-CONCEPTS"
- "SRC-APACHE-ARROW-COLUMNAR"
- "SRC-W3C2013-PROV"
- "SRC-LLVM-MLIR-IR"
artifacts: []
uncertainty: "No pilot, cross-engine benchmark, measured overhead or GONI runtime qualification has been executed. The finite fixture suite can demonstrate behavior for covered semantics only; it cannot prove equivalence for arbitrary formats, programs, model behavior or side effects."
legacy: []
---

# EXP-SEM-PORT-01 — Semantic portability qualification

**Status:** proposed experiment; no measurements performed. Executable fixtures and outputs belong in [goni-prototype-lab](https://github.com/duracell04/goni-prototype-lab), not in the blueprint.

## Hypothesis

For a defined subset of structured business assertions and deterministic calculations, a small typed semantic contract can make independent serialization and execution implementations agree on meaningful results **without material performance or governance cost** relative to the existing Arrow-native approach. This may be false; the experiment compares alternative designs.

## Scope: one falsifiable workflow

Take a supplier offer (the original source is retained), identify an offer/version, extract a price and VAT assertion with currency, unit and validity context, calculate net price and contribution in defined decimal arithmetic, and render a concise decision table. No supplier price in the fixtures represents a verified real offer.

Example illustrative inputs:

```text
gross_price  = Money(149.00, CHF)
tax_rate     = Decimal(0.081)          // 8.1%, assumption
landed_cost  = Money(35.00, CHF)
acquisition  = Money(40.00, CHF)
other_cost   = Money(15.00, CHF)
net_price    = round_half_even(gross_price / (1 + tax_rate), CHF 0.01)
contribution = net_price - landed_cost - acquisition - other_cost
```

For these values the proposed expected net price is **CHF 137.84** and contribution **CHF 47.84**. Treat input price and VAT as synthetic fixtures, not current tax or supplier data. The operation specifies precision, rounding order, currency scale, and errors for missing or invalid inputs.

## Experimental arms

| Arm | Representation and execution | Question |
| --- | --- | --- |
| A — existing baseline | Arrow-native records and direct existing-language calculation | How costly and accurate is the smallest current approach? |
| B — semantic contract | Typed logical record with JSON and Arrow adapters; independently implemented decimal calculations in two engines where their semantics match | Does the additional semantic definition improve portability and correctness at acceptable cost? |
| C — optional graph projection | Same typed assertions projected through RDF-compatible graph syntax with explicit reified statement/provenance support | Does a graph view improve claim/query usefulness without losing source, time or permission metadata? |

A fixed textual Markdown rendering is a **presentation artifact**, not a parallel mutable canonical source. Adding a format to the comparison requires a specified decoder/encoder and loss policy. `RDF` graph syntax and `CSV` are not assumed lossless for an assertion with provenance and epistemic properties unless their adapters explicitly carry them.

## Corpus and negative controls

Start with small synthetic and consented examples. Distinguish separate identities for supplier entity, offer, assertion, content digest and version.

Fixtures must cover at least:

1. Currency mismatch (USD versus CHF) and missing conversion evidence.
2. Decimal rounding near a half-cent boundary and two implementations with different defaults.
3. Explicit null versus missing versus zero; unsupported type or non-finite number.
4. Timestamp with offset and effective/recorded time, including later supersession.
5. Duplicate items, order-sensitive sequences and set-like assertions with different uniqueness semantics.
6. Source reference missing, contradicted assertion and a generated claim misrepresented as observation.
7. Malformed or incomplete input, schema-version change and intentional lossy export.
8. Data hidden by a principal-specific visibility rule; representation conversion must not widen authorization.
9. A proposal to send an external message despite a missing or revoked capability; no side effect may occur.
10. Uncertain effect outcome (e.g. timeout after send attempt) requiring reconciliation rather than a false “not sent” report.
11. Mutation-heavy workload and a large analytical scan to test physical-layout tradeoffs.
12. New field or operation outside the portable subset, requiring explicit rejection or fallback to an identified implementation.

## Measurement and verification

**Representational fidelity.** For each supported value `x:T`, test `Decode_F(Encode_F(x)) ≡_T x` and cross-format `F -> G -> F`. Define `≡_T` in advance, including type, precision, units, value, ordering, identity, source lineage and required scope. Record every conversion-induced loss; a serialization that cannot preserve a property must not receive a pass by omitting the property from scoring.

**Execution agreement.** For each defined pure operation, test two independent implementations on a frozen corpus and randomized boundary inputs. Exact match applies to constrained decimal arithmetic; errors must match expected categories; floating-point tolerance, if used, is specified in advance. Keep runtime versions, input digests, operation IDs and test seeds. A language-neutral signature alone is insufficient evidence of equivalent computation.

**Provenance and temporal integrity.** Verify separately that original source bytes and their derived assertions remain distinguishable; source hashes provide integrity of bytes, not authenticity or truth. Ensure change history and supersession can be reconstructed for supported transformations.

**Authority preservation.** Use the existing kernel authority contract as oracle. Negative controls must cause no unauthorized writes, no cross-principal retrieval, and no prohibited effects **in the test suite**. This is not a universal absence-of-defects guarantee.

**Resource economics.** Measure added runtime memory, allocations, decoding cost, CPU time, serialization latency, indexing burden, storage, development/maintenance effort, and interactive p50/p95 latency where meaningful. Compare these against end-to-end correctness and accepted task completion, not only raw conversion throughput. Include workloads where an Arrow-only design wins.

## Preregistered decision gates

- The candidate logical interface must preserve every critical declared property in the covered fixtures, with explicit failure for unsupported conversions.
- Independent calculations must agree under the declared numerical and error semantics.
- The permission/side-effect controls must pass the test suite without unauthorized effects.
- Provenance completeness must be demonstrable from input evidence through output.
- Engineering overhead must be justified by a measurable advantage (correctness, portability, provenance or lower total lifecycle cost) relative to Arm A; preregister the acceptable overhead and improvement thresholds before representative runs.

If Arm B offers no measurable benefit over the simpler Arrow-native baseline, retain the existing design. If the candidate succeeds only for a narrow class, specify that class explicitly. Architectural promotion of D-002/D-003 requires a separate decision with actual evaluation evidence.

## Reproducibility and reporting

Preserve a fixture manifest, type/operation specification versions, source licenses, input/output hashes, implementation revisions, environment details, timing methodology, rejected cases and negative results. Report uncertainty and sample size. No claim of completed testing, benchmark superiority, or architecture adoption follows from this document.
