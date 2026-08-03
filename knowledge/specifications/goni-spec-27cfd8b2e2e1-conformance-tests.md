---
id: GONI-SPEC-27CFD8B2E2E1
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: local text generation, analysis, criticism, summarization, and drafting do not require a TOOL-01 capability solely because of content or viewpoint creating an external draft, sending, publishing, paying, deleting, writing durable memory, or actuating a device requires the applicable capability model output, retrieved text, and content classification cannot create or
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: Conformance tests
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- local text generation, analysis, criticism, summarization, and drafting do
  not require a TOOL-01 capability solely because of content or viewpoint
- creating an external draft, sending, publishing, paying, deleting, writing
  durable memory, or actuating a device requires the applicable capability
- model output, retrieved text, and content classification cannot create or
  expand a capability grant
- mutating tool calls must include `intent_summary`, `plan_summary`, and
  `tool_intent`
- delegated tool calls must preserve `interaction_mode`, `work_order_id`,
  `done_contract_hash`, and `clarification_decision`
- mediated outcomes must distinguish autonomous, review, escalated, and denied
  execution
- irreversible actions must not commit without approval evidence or a declared
  two-phase path
- audit records and receipts must agree on `autonomy_mode`,
  `delegation_outcome`, `clarification_status`, and
  `clarification_decision`
- synthetic input must require an actuation grant and capability token
- desktop/browser/vision-mediated tool calls must preserve `boundary_basis`
  when observation, extraction, memory, or actuation boundaries affected the
  decision
