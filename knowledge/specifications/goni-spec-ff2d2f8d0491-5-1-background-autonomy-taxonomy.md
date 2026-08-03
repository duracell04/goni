---
id: GONI-SPEC-FF2D2F8D0491
title: 5.1 Background autonomy taxonomy
type: specification
status: draft
implementation_state: specified_only
proposition: Background autonomy is scheduled system work, not an invisible model loop.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 5.1 Background autonomy taxonomy
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 5.1 Background autonomy taxonomy

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5.1 Background autonomy taxonomy

Background autonomy is scheduled system work, not an invisible model loop. Every
background job MUST carry a task class, trigger, budget, policy hash,
autonomy corridor, receipt tier, and interrupt rule.

Required background job families:

| Family | Purpose | Typical trigger | Receipt expectation |
| --- | --- | --- | --- |
| `open_loop_detection` | Detect waiting tasks, stale commitments, unanswered messages, and blocked Work Orders. | timers, event deltas, inbox/calendar changes | summary receipt plus action receipts for proposed interventions |
| `memory_consolidation` | Distill episodic records into semantic/project/procedural memory and tombstone stale entries. | idle window, nightly schedule, explicit request | memory diff receipt and retrieval/index refs |
| `periodic_audit` | Sample autonomous actions, policy drift, receipt chain health, and sandbox denials. | scheduled compliance window | audit summary receipt with checked scope |
| `model_evaluation` | Run approved eval packs against candidate or active bundles. | install, promotion request, periodic regression | EvalReceipt linked to model manifest |
| `policy_drift_check` | Compare active policies, corridors, and capability grants against approved packs. | policy edit, dependency update, schedule | policy receipt and drift findings |
| `scheduled_brief` | Produce Daily Brief, anomaly feed, and pending approval summary. | user schedule or event threshold | brief receipt with source refs |

Background jobs MUST yield to interactive work according to scheduler policy and
MUST NOT create hidden queues outside the Control Plane. Any background job
that proposes external side effects re-enters the normal Work Order, corridor,
tool, sandbox, and receipt path.
