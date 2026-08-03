---
id: GONI-SYNTHESIS-DC66DA308CAE
title: 2.1a Harness observability
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Agentic Harness Engineering maps onto Goni as **Harness Governance**: the system improves by changing observable operating artefacts, not by letting hidden prompts or tool glue drift.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/50-learning-loop.md
  heading: 2.1a Harness observability
  revision: facf4ec813a02ec315fbe482a25bdac18686846e
---

# 2.1a Harness observability

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2.1a Harness observability
Agentic Harness Engineering maps onto Goni as **Harness Governance**: the
system improves by changing observable operating artefacts, not by letting
hidden prompts or tool glue drift.

The harness is governed through three observability requirements
[[lin2026-agentic-harness-engineering]]:

- **Component observability:** prompts, policies, tool manifests, retrieval
  rules, routing rules, approval corridors, context templates, and receipt
  formats are file-backed, versioned, and revertible.
- **Experience observability:** receipts, user edits, approvals, overrides,
  failed retrievals, wrong model routes, latency, cost, and rollback events are
  compressed into experience digests instead of replaying raw trajectory logs
  into the model.
- **Decision observability:** every harness edit declares its prediction, eval
  window, measurement signals, retention criteria, and rollback condition.

Example derived artefacts:

```yaml
experience_digest:
  task_class: "social_follow_up"
  outcome: "approved_after_minor_edit"
  assumption_error: false
  question_needed: false
  policy_change_candidate: "use softer reminder template for weak ties"
  evidence_refs:
    - receipt_id: "rec_..."
    - draft_diff_id: "diff_..."
```

```yaml
harness_change:
  id: "followup-template-exit-ramp-v2"
  target_component: "social_open_loop_policy"
  prediction:
    approval_rate: "+10%"
    user_edit_distance: "-20%"
    negative_feedback_rate: "no increase"
  eval_window: "next 30 follow-up drafts"
  rollback_condition: "negative_feedback_rate > baseline + 5%"
```

These examples are evaluation artefacts, not receipt-schema fields. Receipts
remain the canonical source evidence; harness digests are derived summaries used
for P0/P1/P2 promotion gates.
