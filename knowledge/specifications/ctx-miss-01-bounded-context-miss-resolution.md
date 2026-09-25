---
id: CTX-MISS-01
title: Bounded Context-Miss Resolution
type: specification
status: draft
implementation_state: specified_only
proposition: When model cognition identifies an evidence or procedural dependency absent from the active working set, it may issue a typed bounded context-miss request that re-enters governed retrieval and context compilation without expanding execution authority.
domains:
- memory
- software
- system
- kernel
aliases:
- semantic-page-fault
relations:
- type: depends_on
  target: CTX-COMP-01
- type: depends_on
  target: MEM-RETR-01
- type: depends_on
  target: INF-FRAME-01
- type: depends_on
  target: GONI-DECISION-FEB39440C565
sources:
- SRC-PACKER2023-MEMGPT
artifacts: []
uncertainty: Context-miss detection and recovery quality are unvalidated. The analogy to virtual-memory faults is architectural; Goni requires explicit evaluation of false misses, unnecessary page-ins, recursion, latency, and answer quality.
legacy: []
---

# Bounded Context-Miss Resolution

A compiled working set can be sufficient at invocation time and still prove
insufficient during reasoning. Goni provides a typed read-side recovery path
instead of treating missing evidence as permission to guess.

[
	ext{insufficient active context}
ightarrow
	ext{ContextMiss}
ightarrow
	ext{governed retrieval}
ightarrow
	ext{recompilation}
ightarrow
	ext{resume or stop}.
]

A ContextMiss is a request for additional cognitive material. It is not a tool
capability and does not expand execution authority.

## 1. Logical request

A conforming request MUST preserve:

```yaml
context_miss_id:
work_order_id:
inference_frame_id:
need_type:
query_or_ref:
reason_code:
required_fidelity:
required_confidence:
permission_scope:
token_budget_remaining:
latency_budget_remaining:
recursion_depth:
created_at:
provenance:
```

## 2. Bounded reason codes

The request SHOULD use a bounded reason vocabulary. Initial values are:

- `missing_evidence`
- `missing_exact_wording`
- `unresolved_entity`
- `stale_information`
- `conflicting_evidence`
- `missing_procedural_rule`

The request records what additional material is required without requiring raw
private chain-of-thought disclosure.

## 3. Resolution path

A context miss re-enters the normal governed pipeline:

1. validate that the Work Order permits further read-side retrieval;
2. check recursion, token, latency, privacy, and connector budgets;
3. issue or refine a governed retrieval request through MEM-RETR-01;
4. apply ordinary permission, validity, quoteability, trust, and conflict
   filters;
5. invoke CTX-COMP-01 to produce a new ContextPack or bounded ContextPack delta;
6. serialize a new InferenceFrame under INF-FRAME-01;
7. resume cognition if budgets and policy permit;
8. emit a receipt for the miss, recovery basis, materialization, and outcome.

## 4. Termination

The runtime MUST have a finite recursion bound. Resolution terminates when:

- sufficient material is recovered;
- the remaining budget is insufficient;
- policy denies the read;
- the required source is unavailable;
- confidence remains below the Work Order threshold after the permitted
  recovery attempts.

When recovery terminates without sufficient evidence, the system SHOULD surface
the unresolved uncertainty rather than silently filling the gap.

## 5. Authority invariant

[
oxed{
	ext{context expansion}

otRightarrow
	ext{authority expansion}
}
]

A newly retrieved policy description, mandate summary, cached permission hint,
or capability reference may inform cognition. Consequential execution still
resolves authority against canonical kernel-owned state at the mediation
boundary.
