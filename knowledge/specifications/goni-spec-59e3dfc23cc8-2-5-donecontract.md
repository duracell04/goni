---
id: GONI-SPEC-59E3DFC23CC8
title: 2.5 DoneContract
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every executable turn MUST have a DoneContract with deliverable, must_include, must_verify, and stop_condition. The DoneContract is the kernel-visible statement of what counts as finished, and consequential completion requires qualifying post-action evidence for the required verification predicates rather than tool or provider success alone.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: The completion-certification semantics are specified architecture only. Concrete verifier implementations, evidence-strength thresholds, and independence policies require evaluation.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 2.5 DoneContract
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 2.5 DoneContract

> Status boundary: this is a specified-only contract. Enforcement language
> describes intended conformance behavior rather than observed implementation.

### 2.5 DoneContract

Every executable turn MUST have a `DoneContract` with:

- `deliverable`
- `must_include`
- `must_verify`
- `stop_condition`

The DoneContract is the kernel-visible statement of what counts as finished. It
must be hashable, stable across retries, and compact enough to reference in
receipts and audit records.

## Completion semantics

`must_verify` defines the postconditions that must hold before Goni may treat
the delegated objective as completed. Each consequential verification
requirement SHOULD identify, directly or by stable reference:

- the predicate or state that must hold;
- the observation path capable of evaluating that predicate;
- the minimum evidence required to support the result;
- any independence requirement between the action path and verification path.

The core invariant is:

> **Execution success is not task completion.**

A successful tool call, command exit code, provider acknowledgement, queued
state, deployment state, API status, or other intermediate control-plane signal
MUST NOT by itself satisfy the DoneContract unless the contract explicitly
defines that signal as the desired terminal state.

The runtime therefore distinguishes at least:

1. **action outcome** — whether the attempted operation succeeded at its own
   interface;
2. **observed state** — what post-action evidence shows about the resulting
   system or world state;
3. **DoneContract satisfaction** — whether all required postconditions are
   supported by qualifying evidence.

A `stop_condition` determines when further work should cease. It does not
silently convert an unverified or unknown outcome into successful completion.

If an action succeeds but required verification cannot be performed, the task
remains incomplete from the perspective of the DoneContract. The runtime must
preserve that distinction for completion certification under `COMPLETE-01`.

For `audit_grade` work, the DoneContract MUST also identify:

- the minimum evidence scope required for the conclusion;
- the allowed strength of negative claims;
- whether missing evidence must be surfaced before completion;
- any required independent or read-after-write verification path.
