---
id: GONI-SPEC-7D0EF565DF3C
title: Receipt tiers
type: specification
status: draft
implementation_state: specified_only
proposition: Receipt volume must not drown governance evidence.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/receipts.md
  heading: Receipt tiers
  revision: 0b6bf1bf99eef10258d5ea44c7c90bdc24542c70
---

# Receipt tiers

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Receipt tiers

Receipt volume must not drown governance evidence. Goni therefore distinguishes
receipt tier from ordinary telemetry:

| Tier | Purpose | Examples | Retention posture |
| --- | --- | --- | --- |
| `governance` | Authority, approval, policy, or irreversible action evidence. | approvals, denied actions, external side effects, model promotion, policy override | durable, hash-chained |
| `execution` | Normal mediated work evidence. | tool call, model route, memory read/write, parser-mediated ingestion | durable or compactable by policy |
| `summary` | High-volume background or batch rollups. | index rebuild summary, scheduled audit summary, Daily Brief generation | durable summary with refs to sampled spans |
| `telemetry` | Performance and health signals. | latency, queue depth, cache hit, thermal signal | metrics store; not a substitute for receipts |

The receipt tier MUST be derivable from `action_type`, `task_class`, policy
decision, risk class, and side-effect class. Governance-tier events may not be
silently downgraded to telemetry. High-volume background events may use summary
receipts if the summary preserves checked scope, inputs/outputs by hash, policy
decision, sampled span refs, and failure counts.

Learning receipts produced by the Correction Delta Compiler follow the same
tier rules. A proposed low-risk hypothesis MAY use an execution-tier receipt. An
accepted, rejected, promoted, high-risk, authority-affecting, or policy-affecting
learning update MUST use a governance-tier receipt. Learning receipts MUST
record source refs, detected delta summary, proposed rule, scope, confidence,
evidence count, contradiction count, review status, memory diff refs, and
regression test refs without storing raw draft or user-correction text by
default.

Model adaptation receipts follow the same governance-tier rule when model
behavior is changed, promoted, rolled back, or made eligible for a new runtime
route. This includes adapter promotion, LoRA/QLoRA/DPO-style preference
artifacts, prompt/policy bundle promotion, worldview or task-lens activation,
and any model-stack change that affects private memory, tool access, or default
behavior. Adaptation receipts MUST record base bundle refs, adapter or bundle
identity, training or preference dataset refs, training config refs, eval refs,
expected behavior change summary, approval status, active runtime selection,
and rollback ref. They must store hashes, refs, summaries, and policy decisions
by default, not raw private prompts, memory text, or training examples.
