---
id: GONI-SPEC-E21B4752C48D
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: any mediated action must emit exactly one receipt receipts must form a valid hash chain receipts must omit raw content by default viewpoint or content classification alone must not elevate receipt tier, create tool authority, or expand monitoring receipt collection must remain scoped to governed system transitions rather
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/receipts.md
  heading: Conformance tests
  revision: 0b6bf1bf99eef10258d5ea44c7c90bdc24542c70
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- any mediated action must emit exactly one receipt
- receipts must form a valid hash chain
- receipts must omit raw content by default
- viewpoint or content classification alone must not elevate receipt tier,
  create tool authority, or expand monitoring
- receipt collection must remain scoped to governed system transitions rather
  than ambient human activity
- durable memory commits, ontology merges/splits, and controlling-rule changes
  must preserve source, authority, policy, diff, and undo refs using existing
  receipt fields
- receipts must include `trace_id`, `span_id`, `decision_basis`, and
  `memory_read_refs` and `memory_diff_refs`
- retrieval-mediated receipts must include `retrieval_basis` when retrieved
  memory affected output or execution
- model-routing receipts must include `llm_route` when model selection affected
  output or remote escalation eligibility
- model-stack receipts must include `model_stack_basis` when adapters, prompt
  policy, memory bundle refs, or eval refs affected output or eligibility
- receipts must include `task_class`, `autonomy_mode`, `risk_score`, and
  `risk_basis` for any delegated action
- delegated/tool-mediated receipts must preserve `interaction_mode`,
  `work_order_id`, `done_contract_hash`, `clarification_decision`, and
  `objective_option_count`
- delegated/tool-mediated receipts must include `delegation.assumptions`,
  `delegation.uncertainty_level`, `delegation.question_strategy`,
  `delegation.tool_intent`, `delegation.delegation_outcome`, and
  `delegation.undo_strategy_ref`
- receipt `decision_basis` must preserve `intent_summary` and `plan_summary`
  when a mutating action is proposed or executed
- third-party framework logs or audit events must not be accepted as the sole
  terminal record of a mediated effect
- governance-tier events must not be downgraded to telemetry-only records
- parser-mediated memory/context changes must preserve `parser_basis`
- correction-derived learning changes must preserve `learning_basis`
  sufficient to audit the update without raw content
- model adaptation changes must preserve `adaptation_basis` sufficient to audit
  base bundle, adapter, dataset refs, eval refs, approval, active runtime
  selection, and rollback without raw private training content
- visual actions must preserve `visual_basis` with source hashes, workflow
  hash, output hash, verification summary, and rollback ref where applicable
- visual receipts must omit raw private screenshots, image binaries, full OCR
  text, and unbounded prompts by default
- desktop/browser/vision-mediated actions must include `boundary_basis` when a
  boundary transition affected observation, extraction, memory, actuation,
  sandboxing, approval, egress, or rollback/repair
- denied boundary transitions must emit an auditable receipt or receipt-linked
  record without storing raw private content by default
- robot-mediated actions must include `robot_basis` when robot observation,
  extraction, memory, physical actuation, egress, remote supervision, safety
  envelopes, or verification affected the action
- robot receipts must omit raw private maps, raw video, raw audio, full sensor
  logs, unrestricted telemetry, and unbounded transcripts by default
