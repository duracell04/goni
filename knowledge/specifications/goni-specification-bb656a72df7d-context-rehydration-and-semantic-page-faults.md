---
id: GONI-SPECIFICATION-BB656A72DF7D
title: Context Rehydration and Semantic Page Faults
type: specification
status: draft
implementation_state: specified_only
proposition: When information required for a consequential reasoning step is absent from active context but recoverable from governed memory or authoritative state, the system should retrieve and rehydrate that information before substituting an unsupported reconstruction.
domains:
- memory
- retrieval
- architecture
aliases:
- semantic page fault
relations:
- type: refines
  target: GONI-THESIS-AEA4F8746318
- type: depends_on
  target: GONI-SPECIFICATION-C64D8275BA34
sources:
- SRC-PACKER2023-MEMGPT
- SRC-GOOGLE2026-LITERT-LM
artifacts: []
uncertainty: Detection quality, acceptable miss rates, and page-in policies remain empirical questions. The term semantic page fault is a GONI design abstraction rather than a claim of established terminology.
legacy: []
---

# Context Rehydration and Semantic Page Faults

A **semantic page fault** is a cognitive miss: the system identifies that information needed for the current reasoning step is absent from active context but may exist in recoverable state.

The intended transition is:

`miss → retrieve → rehydrate → re-evaluate`

rather than:

`miss → guess`

Candidate triggers include uncertainty about a recoverable fact, a reference to an earlier decision whose rationale is absent, a missing exact identifier or file state, conflict between a summary and pinned state, a consequential action that requires source evidence, or detection that a compressed representation omitted an invariant needed for the next step.

Rehydration should recover the smallest sufficient evidence-bearing state. The router may combine semantic similarity, lexical matching, exact identifiers, structural links, temporal signals, dependency information, current-plan state, file paths, Git references, and policy metadata. A useful abstraction is:

`R(q,s) = f(R_semantic, R_lexical, R_structural, R_temporal, R_dependency, R_plan)`

Lossy summaries may assist routing, salience estimation, and context assembly, but should retain provenance pointers and should not silently supersede the underlying source. In compact form:

**Summaries are indexes; sources remain the evidence-bearing memory.**

Rehydration itself grants no execution permission. Retrieved state enters reasoning subject to the same provenance, retention, confidentiality, and authority checks that applied before paging.
