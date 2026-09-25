---
id: CTX-COMP-01
title: Authority-Aware Context Compilation
type: specification
status: draft
implementation_state: specified_only
proposition: The Context Plane compiles a task-specific, provenance-bearing working set from governed evidence, memory, skills, policy-visible state, and runtime budgets while keeping execution authority independent from model-visible context.
domains:
- memory
- software
- system
- kernel
aliases:
- context-compiler
relations:
- type: depends_on
  target: MEM-RETR-01
- type: depends_on
  target: CGG-01
- type: depends_on
  target: SKILL-REG-01
- type: depends_on
  target: GONI-DECISION-9AF49466170D
- type: refines
  target: GONI-SPEC-2F762878317A
- type: refines
  target: GONI-SPEC-B49DB23CF412
- type: refines
  target: GONI-PRINCIPLE-9062425CD490
sources:
- SRC-DENNING1968-WORKING-SET
- SRC-JIANG2023-LLMLINGUA
- SRC-JIANG2024-LONGLLMLINGUA
- SRC-LI2023-SELECTIVE-CONTEXT
artifacts: []
uncertainty: The compiler interface and invariants are specified architecture. The objective weights, fidelity thresholds, selection algorithms, and transferability of published compression results require Goni-specific implementation and matched evaluation.
legacy: []
---

# Authority-Aware Context Compilation

Goni treats model context as a compiled working set rather than as durable
memory. The Context Plane remains the canonical plane; the Context Compiler is a
service inside that plane, not an additional architectural plane.

The governing separation is:

[
oxed{
	ext{durable state}

eq
	ext{compiled semantic context}

eq
	ext{runtime KV state}
}
]

## 1. Interface

For Work Order (W_t), governed retrieval candidates (R_t), durable memory
references (M_t), eligible skill fragments (S_t), visibility constraints
(P_t), resource budget (B_t), and target runtime profile (ho_t), the
Context Plane computes:

[
C_t =
operatorname{CompileContext}
(W_t,R_t,M_t,S_t,P_t,B_t,ho_t).
]

The output (C_t) is a `ContextPack`.

Inputs may include hashes or stable refs rather than raw content when the
compiler can resolve them through governed interfaces.

## 2. Compilation stages

A conforming compiler SHOULD preserve the following logical stages:

1. bind all context work to a Work Order;
2. ingest governed retrieval candidates;
3. expand through permitted graph relations where CGG-01 applies;
4. filter by permission, validity, conflict state, quoteability, source trust,
   parser confidence, and other visibility rules;
5. resolve eligible skills and close fragment dependencies under SKILL-REG-01;
6. score and rerank candidate evidence and procedural material;
7. reduce redundancy and perform budgeted selection;
8. choose a permitted compression form for each selected item;
9. order the selected material deterministically under a named compiler policy;
10. emit a model-independent `ContextPack` with inclusion, omission,
    compression, provenance, and receipt metadata.

Retrieval produces candidate material. Retrieval rank does not by itself define
the final model context.

## 3. Smallest-sufficient-context objective

The design objective is not shortest context in isolation. A conceptual
objective is:

[
C_t^* =
argmin_C
[
alpha T(C)
+eta L(C)
+gamma M_{KV}(C)
+delta E(C)
+epsilon D(C)
],
]

where (T) is active-token cost, (L) latency, (M_{KV}) expected runtime
KV-state cost, (E) compute or energy cost, and (D) task-relevant
information distortion or loss.

The exact scoring function is not normative. Any promoted implementation MUST
instead demonstrate named quality and safety thresholds, including:

[
P(	ext{task success}mid C) ge 	au,
]

and sufficient coverage of evidence required by the Work Order.

A shorter context that omits decisive evidence is a compiler failure even when
it reduces cost.

## 4. Authority boundary

Model-visible information never constitutes execution authority.

[
oxed{
operatorname{InformationAvailableToModel}

otRightarrow
operatorname{AuthorityToAct}
}
]

Compression, retrieval rank, cache state, skill selection, summaries, inferred
salience, and ContextPack contents may influence cognition. Consequential
execution MUST resolve mandates, policy, capabilities, approval state, budgets,
and revocation against canonical kernel-owned authority according to
GONI-DECISION-9AF49466170D.

## 5. Provenance and replay

A ContextPack MUST identify the compiler version or policy, source snapshots or
refs sufficient for reconstruction, selected items, material omissions that
cross the configured audit threshold, compression choices, skill-fragment refs,
budget, and a stable ContextPack hash.

Context compilation may be probabilistic during candidate generation or
scoring. Audit-grade profiles SHOULD support deterministic ordering and
replayable materialization once candidate inputs and named policies are fixed.

## 6. Working-set interpretation

Denning's working-set model supplies an operating-systems precedent for managing
a small active set within a larger addressable state. Prompt-compression
research supplies empirical evidence that indiscriminate context length is not
the only useful objective. Goni combines these ideas with governed memory,
skills, provenance, and authority separation; that combination is a Goni
architectural hypothesis requiring system-level evaluation.
