---
id: "GONI-PROP-COMP-01"
title: "Proposed typed computation, effect and execution-evidence semantics"
type: "proposal"
status: "draft"
implementation_state: "specified_only"
proposition: "Investigate a small typed contract distinguishing pure deterministic transformations, fallible computations, probabilistic inference and externally effectful operations, with interchangeable implementation engines and kernel-owned authority and outcome verification."
domains:
- "agent"
- "kernel"
- "research"
- "software"
- "system"
aliases:
- "portable-typed-computation"
relations:
- type: "derived_from"
  target: "EVID-SEMANTIC-FOUNDATIONS-2026"
- type: "depends_on"
  target: "GONI-PROP-SEM-01"
- type: "depends_on"
  target: "SS-01"
- type: "depends_on"
  target: "TOOL-01"
- type: "depends_on"
  target: "SPEC-TXN-01"
sources:
- "SRC-PIERCE2002-TAPL"
- "SRC-PARNAS1972-INFORMATION-HIDING"
- "SRC-LLVM-MLIR-IR"
- "SRC-WASM-WIT-INTERFACES"
- "SRC-W3C2013-PROV"
artifacts: []
uncertainty: "Typed interfaces alone do not guarantee equivalent behavior. A portable operation vocabulary requires a constrained set of defined semantics and conformance testing; arbitrary code, probabilistic models, side effects and external state cannot be made engine-independent merely by serializing signatures."
legacy: []
---

# GONI-PROP-COMP-01 — Computation, effects and evidence

**Status: proposed, specified only.** Describing computation as data is distinct from executing it. This document proposes semantic contracts across the existing Data, Context, Control and Execution planes. It changes neither the current tool API nor permission semantics.

## Four operational classes

| Class | Illustrative signature | Meaning and verification |
| --- | --- | --- |
| Pure deterministic | `f : A -> B` | Defined input/output types and mathematical behavior, no external state mutation. |
| Partial/fallible | `f : A -> Result(B, Error)` | Invalid values, unrepresentable outputs, overflow and unavailable data are explicit cases. |
| Probabilistic | `p : (A, Model, Parameters, Randomness) -> Distribution(B)` | Sampling and model dependencies identified; compare measured behavior and distributions rather than assume output equality. |
| Effectful | `effect : (Command, State, Authority) -> (NewState, Outcome, Receipt)` | Kernel mediates permissions and state transitions; output verification may remain unknown after a timeout. |

These are semantic *classifications*, not a requirement to implement four programming frameworks. Model state, randomness, nondeterministic hardware kernels and external service responses must be identified at the right granularity.

## Small computation expression subset

A representative portable expression grammar is:

```text
Expr(T) = Constant(T) | Input(Ref(T))
        | Apply(OperatorId, Version, [TypedArg])
        | Let(Name, Expr, Expr)
```

Operators define argument types, result/error types, rounding behavior, overflow policy and evaluation order where observable. A computation may form a typed DAG with referenced subexpressions. `Apply` is executable only if a verified implementation of the named operation exists for the selected backend.

An example of the gross-to-net calculation:

```text
round_money(
  divide_decimal(gross_price, add_decimal(1, tax_rate)),
  currency_scale=2,
  rounding=HALF_EVEN
)
```

The code above is **illustrative logical pseudocode**, not a shipped GONI IR syntax. The VAT rate, price basis, currency, effective date, rounding choice and price source are indispensable context for financial meaning.

A normal program may still be implemented directly in Python, Rust, SQL or another language. This proposal recommends a *small portable subset* for recurring deterministic calculations, with other operations registered as versioned typed services. Supporting new languages requires new adapters and checks, not rewriting the canonical mathematical definition.

## Equivalent does not mean identical bytes

For a declared deterministic operation `f` and conforming backends `A` and `B`, the proposed test is:

```text
Decode_A(Eval_A(Encode_A(f, x))) ==_T
Decode_B(Eval_B(Encode_B(f, x)))
```

The equivalence relation `==_T` is predeclared: exact for constrained integers/decimal money and identifiers; tolerance or interval-based for specified floating-point computations; explicit result-category equivalence for errors. If the backends differ in overflow, Unicode collation, SQL NULL semantics, timezone conversions or rounding, their operations are *not automatically equivalent*. Cross-engine portability must be scoped to an agreed observable subset.

## Effects and authority remain outside the pure calculus

Do not convert a document's assertion `may_send_email=true`, a model's proposed tool call, or a successful pure calculation into execution permission. Existing GONI `SS-01`, `TOOL-01` and `SPEC-TXN-01` own the respective validation, capability and transactional semantics. Every effect is checked against current kernel-owned mandate, scope, budget, approval and revocation state.

For a requested action, preserve distinguishable states:

```text
Proposed -> Authorized -> Attempted -> Acknowledged -> Verified
                                     \-> Failed / Unknown
```

A timeout after invocation may be **unknown outcome**, not proof that a remote message was not sent. Where available, use idempotency and remote reconciliation before retry. A receipt documents what was *observed*, and cannot prove unobserved downstream delivery or truth.

## Provenance and reproducibility

A computation result should identify source assertion IDs, operation ID/version, schema version, execution engine, parameter set, meaningful environmental constraints and execution trace or receipt. A cryptographic content hash proves identity of bytes under a hash assumption, not truth, compliance or authorization. W3C PROV gives interoperable derivation concepts, not a substitute for kernel checks.

For probabilistic inference, record model revision, parameters, inputs, tools and available nondeterminism signals. Some evaluations can compare distributions or held-out behavior even when exact output replay is not guaranteed. For effects, record requested versus observed outcomes and keep uncertainty.

## Independent implementations and minimality

[MLIR](https://mlir.llvm.org/docs/LangRef/) demonstrates a typed operation/value representation at compiler level; [WIT](https://component-model.bytecodealliance.org/design/wit.html) defines portable interfaces but explicitly leaves behavior to implementations. These are *precedents*, not a prescription to embed either entire system.

Implementations may vary. To make a shared contract meaningful, test their observable behavior rather than infer equivalence from naming, method signatures or vendor claims.

**Boundary:** no mandatory new interpreter, universal metaprogramming system, general orchestration graph or permanent service is approved here. Introduce the smallest form needed to verify one complete calculation pipeline; extend only with demonstrated benefit.

## Research links

`GONI-PROP-SEM-01` supplies typed values and identities; `GONI-PROP-ARCH-01` studies physical placement and D-002/D-003 interpretation; `EXP-SEM-PORT-01` proposes implementation-independent qualification.
