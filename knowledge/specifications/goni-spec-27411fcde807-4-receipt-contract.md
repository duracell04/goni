---
id: GONI-SPEC-27411FCDE807
title: 4. Receipt contract
type: specification
status: draft
implementation_state: specified_only
proposition: Every mediated action that reads memory MUST include memory_read_refs in its receipt.
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
  heading: 4. Receipt contract
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# 4. Receipt contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Receipt contract

Every mediated action that reads memory MUST include `memory_read_refs` in its
receipt. Use an empty list when no memory was read.

Every mediated action that changes memory MUST continue to include
`memory_diff_refs`. Use an empty list when no memory mutation occurred.

When retrieval affects output or execution, receipts SHOULD include
`retrieval_basis` with:

- retrieval mode (`dense | sparse | hybrid | graph | mixed`),
- index refs or versions,
- reranker id or policy,
- graph snapshot/config refs, traversal depth, scoring policy, and decay policy
  when graph traversal is used,
- ContextPack refs, inclusion reasons, omission reasons, and compression policy
  when graph traversal compiles a ContextPack,
- selected context refs,
- source trust and permission filters,
- policy hash.

Receipt fields MUST NOT store raw source text by default.

When parsing affects memory, context, or execution, receipts SHOULD include
`parser_basis` with:

- source hash and source URI/ref,
- parser ID and parser version,
- parsed structure kind (`text | table | form | email | calendar | code |
  mixed`),
- chunk boundary refs,
- confidence flags and extraction warnings,
- produced chunk IDs or memory IDs,
- policy hash and permission filters.

Parser receipts MUST NOT store raw extracted text by default. They store hashes,
refs, structure summaries, and confidence metadata sufficient to replay or
challenge the parse.
