---
id: GONI-SPEC-67C20FD1E8A7
title: Receipt completeness profile (normative)
type: specification
status: draft
implementation_state: specified_only
proposition: trace_id identifies the request/run trace for correlation across spans.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/receipts.md
  heading: Receipt completeness profile (normative)
  revision: 0b6bf1bf99eef10258d5ea44c7c90bdc24542c70
---

# Receipt completeness profile (normative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Receipt completeness profile (normative)
- `trace_id` identifies the request/run trace for correlation across spans.
- `span_id` identifies the exact operation span that emitted the receipt.
- `decision_basis` captures the basis for policy mediation (for example:
  `policy_hash`, matching rule IDs, approval references, `intent_summary`, and
  `plan_summary`).
- `memory_diff_refs` is a list of state/memory delta IDs caused by the action.
  Use an empty list when no memory mutation occurred.
- `memory_read_refs` is a list of memory IDs, chunk IDs, context item IDs, or
  retrieval result refs read by the action. Use an empty list when no memory
  was read.
- `retrieval_basis` records the retrieval mode, index refs, selected context
  refs, reranker, permission filters, and policy hash when retrieval affected
  output or execution. It must not store raw retrieved text by default.
- `bundle_id`, `manifest_hash`, and `eval_receipt_refs` record governed model
  bundle provenance when a model route depends on an approved bundle.
- `model_stack_basis` records active base bundle, adapter set, system
  prompt/policy version, memory or retrieval bundle refs, eval receipt refs,
  and route policy hash when those choices affected output or tool eligibility.
- `llm_route` records model-routing decisions when an LLM path is selected.
  It captures the selected route, local/Council rationale, task
  classification, models considered and used, redaction requirement, privacy
  class sent, cost/latency estimate labels, confidence label, and policy
  decision. It must not store raw prompt or retrieved text.
- `assurance_level`, `ml_bom_ref`, and `attestation_refs` record the local
  installation trust state when model bundle governance affects the route.
- `receipt_tier` records the governance/execution/summary/telemetry tier used
  for retention and review policy.
- `parser_basis` records parser identity, source hash, structure kind,
  confidence flags, chunk refs, and policy filters when parsing affected memory
  or context.
- `visual_basis` records source asset hashes and refs, model bundle ID,
  manifest hash, workflow hash, compact prompt summary, mask refs, control
  refs, transformation summaries, verification results, output hash, and
  rollback ref when visual analysis, generation, editing, or memory updates
  affected the action. It must not store raw private images, raw screenshots,
  full OCR text, or unbounded prompts by default.
- `learning_basis` records Correction Delta Compiler outputs when correction
  deltas affect memory, retrieval, prompt assembly, harness policy, or
  promotion datasets. It captures detected delta class, proposed rule ref,
  scope, confidence, evidence and contradiction counts, review status,
  regression refs, source refs, and policy hash. It must not store raw draft or
  correction text by default.
- `adaptation_basis` records governed model adaptation when a model stack,
  adapter, preference dataset, prompt/policy bundle, or worldview/task lens is
  changed or promoted. It captures base bundle refs, adapter refs, training or
  preference dataset refs, training config refs, expected behavior summary,
  evaluation refs, approval refs, active runtime selection, policy hash, and
  rollback ref. It must not store raw private prompts, memory text, or training
  examples by default.
- `boundary_basis` records Desktop Agent Firewall decisions when observation,
  extraction, memory, actuation, sandbox, approval, rollback/repair, or remote
  extraction boundaries affected the action. It captures refs and compact
  policy decisions, not raw private screen content, OCR text, accessibility
  dumps, audio transcripts, or unbounded prompts.
- `robot_basis` records embodied robot decisions when robot observation,
  extraction, memory, physical actuation, egress, remote supervision, safety
  envelopes, or verification affected the action. It captures robot refs,
  adapter refs, mandate refs, environment scope refs, actuation grant refs,
  sensor basis refs, policy decisions, verification summaries, intervention
  state, and rollback/repair refs. It must not store raw private maps, raw
  video, raw audio, full sensor logs, unrestricted telemetry, or unbounded
  transcripts by default.
- `interaction_mode` records whether the turn was delegated execution or
  co-creation.
- `work_order_id` references the canonical pre-execution Work Order.
- `done_contract_hash` references the hashed completion contract used for the
  action.
- `clarification_decision` records the control-plane branch:
  `assume | ask_decisive | propose_objectives | block`.
- `objective_option_count` records how many candidate objectives were surfaced,
  if any.
- `delegation.assumptions` lists material assumptions surfaced to the operator
  or carried forward by policy.
- `delegation.uncertainty_level` records the confidence band used for the
  execution decision.
- `delegation.question_strategy` records whether clarification was skipped,
  asked decisively, or overridden by the user/policy.
- `delegation.tool_intent` captures the purpose of the concrete tool action,
  distinct from the user-level goal.
- `delegation.delegation_outcome` records whether the action was executed,
  queued for review, blocked, escalated, or approved.
- `delegation.undo_strategy_ref` records the rollback/compensation strategy
  reference when one exists.
