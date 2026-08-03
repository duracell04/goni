---
id: GONI-SPEC-631D7FAF2790
title: 1. Tool call envelope
type: specification
status: draft
implementation_state: specified_only
proposition: 'Logical fields for every tool call: tool_id args agent_id capability_token_id state_snapshot_id policy_hash provenance boundary_basis (refs to observation, extraction, memory, actuation, sandbox, approval, and rollback/repair boundary decisions when a desktop, browser, or vision-mediated surface is involved) operation_id'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: 1. Tool call envelope
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# 1. Tool call envelope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Tool call envelope

Logical fields for every tool call:

- `tool_id`
- `args`
- `agent_id`
- `capability_token_id`
- `state_snapshot_id`
- `policy_hash`
- `provenance`
- `boundary_basis` (refs to observation, extraction, memory, actuation,
  sandbox, approval, and rollback/repair boundary decisions when a desktop,
  browser, or vision-mediated surface is involved)
- `operation_id`
- `task_class`
- `interaction_mode` (`delegation` | `co_creation`)
- `autonomy_mode` (`no_go` | `soft_gate` | `autopilot` | `escalated`)
- `risk_score`
- `risk_basis`
- `work_order_id`
- `intent_summary` (normalized statement of repaired user intent)
- `plan_summary` (bounded step plan for the current action)
- `done_contract_hash`
- `tool_intent` (why this tool call is needed)
- `clarification_decision` (`assume` | `ask_decisive` |
  `propose_objectives` | `block`)
- `clarification_status` (`not_needed` | `asked_decisive` |
  `skipped_with_assumptions` | `user_overrode`)
- `objective_option_count`
- `assumption_refs` (structured references to surfaced assumptions)
- `delegation_outcome` (`autonomous` | `review` | `blocked` |
  `escalated` | `approved`)
- `undo_strategy_ref` (optional reference to compensation/rollback path)
- `idempotency_key` (required for mutating calls)
- `precondition_refs` (state/version references that must hold at commit time)

The kernel validates the call against policy and capability tokens before
execution.
Validation follows the SS-01 arbitration contract (symbolic constraints in F_sparse).

Mutating calls MUST carry a visible chain from repaired intent to concrete tool
proposal. This makes delegation behavior inspectable rather than burying it in
free-form prompts.

Third-party adapters, agent gateways, or external assistant frameworks do not
satisfy this mediation requirement on their own. If they expose tools or
actions to Goni, those effects MUST still pass through Goni-issued capability
tokens and Goni policy evaluation before execution.

Synthetic input is a tool syscall. Mouse, keyboard, scroll, drag, browser DOM
mutation, shell command, filesystem write, external API mutation, and publish
actions are actuation events. They MUST NOT be treated as ambient desktop
authority inherited from a screen-sharing, accessibility, browser, or
computer-use session. If an action starts from observed screen or app context,
BOUND-01 defines the required boundary chain before this tool envelope may
execute.

Robot skills are also tool syscalls when they cause or prepare physical-world
effects. Movement, navigation, grasping, carrying, sorting, cleaning, device
operation, person assistance, security patrol, remote-supervised action, and
robot-cloud command paths require capability-scoped mediation under ROBOT-01.
Robot vendor SDKs, fleet managers, teleoperation systems, or middleware logs do
not create authority without a Goni capability token and policy decision.
