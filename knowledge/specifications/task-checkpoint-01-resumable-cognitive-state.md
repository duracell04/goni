---
id: TASK-CHECKPOINT-01
title: Resumable Cognitive Task Checkpoint
type: specification
status: draft
implementation_state: specified_only
proposition: >-
  Context rollover and task suspension require a bounded, provenance-bearing
  checkpoint that preserves resumable task state without treating a prose chat
  summary or hidden model reasoning as canonical state.
domains:
- context
- memory
- orchestration
aliases:
- COGNITIVE-CHECKPOINT
relations:
- type: depends_on
  target: CTX-LIFE-01
- type: depends_on
  target: MEM-RETR-01
- type: refines
  target: GONI-SPEC-33294E0D3306
sources: []
artifacts: []
uncertainty: This logical shape is specified only and is not yet a shipping schema or API object.
legacy: []
---

# Resumable Cognitive Task Checkpoint

> Status boundary: this is a specified-only logical contract. It does not add a shipping table or API until a later schema revision promotes it.

A task checkpoint preserves the minimum state required to continue a Work Order after context rollover, process suspension, cache eviction, model switching, or device restart. It is not a transcript summary and MUST NOT require storage of hidden chain-of-thought.

A conforming logical checkpoint should preserve the following shape:

```yaml
checkpoint_id:
work_order_id:

objective_ref:
task_state:

decision_refs:
constraint_refs:
open_question_refs:
evidence_refs:
artifact_refs:
commitment_refs:

pending_action_refs:
memory_diff_refs:

prior_context_pack_ref:
source_episode_ref:

created_at:
provenance:
receipt_ref:
```

`task_state` is bounded derived state: a concise description of what has been established, what remains unresolved, and where execution should resume. Decisions, evidence, commitments, and constraints SHOULD use stable references whenever possible rather than duplicating source text.

Creating a checkpoint does not itself grant memory authority. Any durable memory mutation triggered by checkpointing remains subject to the existing memory write gate and provenance rules.

Restoration SHOULD compile a new active context from:

[
\text{checkpoint}
+
\text{current governed memory}
+
\text{current policy}
+
\text{fresh retrieval}.
]

This makes task continuation robust to changes that occurred while the context was inactive. A stale checkpoint can guide reconstruction, but it cannot override newer policy, corrected memory, revoked authority, or contradictory evidence.
