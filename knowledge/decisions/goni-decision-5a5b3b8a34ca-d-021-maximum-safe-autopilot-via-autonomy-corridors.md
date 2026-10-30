---
id: GONI-DECISION-5A5B3B8A34CA
title: D-021 - Maximum safe autopilot via autonomy corridors
type: decision
status: draft
implementation_state: specified_only
proposition: 'Autonomous execution requires a valid human-granted capability, current policy, satisfied preconditions, a permitted corridor, sufficient budget, and required approval; risk thresholds can only restrict a pre-authorized action.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-021 - Maximum safe autopilot via autonomy corridors
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-021 - Maximum safe autopilot via autonomy corridors

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense enforcement language specifies intended behavior and is not evidence of implementation.

## Refined formal decision (2026-10-30 draft)

Autonomy policy remains keyed by task class, corridor, and explicit risk threshold. The corridors are `no_go`, `soft_gate`, and `autopilot`.

The kernel permits an effect only if **all applicable conditions** hold:

1. The action belongs to a current human-granted mandate and an authorized task class.
2. Its capability is authentic, current, scoped to the actual tool and resource, unexpired, and unrevoked.
3. Resource scope, recipient scope, time limits, budgets, and policy preconditions are satisfied.
4. The corridor permits the proposed effect and its risk score falls within its authorized threshold.
5. Any approval required for that particular effect has been granted **before execution**.
6. The action passes the execution mediator and receives the required receipt.

A low risk score **never grants authority**. Model reasoning, newly generated code, agent routing, or an autonomy threshold cannot widen an existing mandate.

## Corridor semantics

- `no_go`: Deny the specified effect. To enable it, an authorized human must first change the underlying policy.
- `soft_gate`: Automatically execute only explicitly authorized low-risk effects designated for **review after execution**. Route approval-required effects for **approval before execution**. Review and approval are separate decisions.
- `autopilot`: Execute effects within the valid mandate, capability, risk, and budget bounds without routine confirmation; escalate exceptions and protected effects.

A separate effect-specific approval requirement applies regardless of the corridor label. Human-facing terms `Restricted`, `Review`, and `Autopilot` may summarize corridors, but the interface must display the effective permissions and whether oversight happens before or after execution.

## Rationale and consequences

The original threshold-only formula expressed the risk filter but omitted the surrounding authority preconditions. This refinement makes the complete permission boundary explicit and aligns it with Goni's existing default-deny policy, capability mediation, and canonical authority model.

Conformance must test expiry, revocation, scope escape, threshold errors, mismatched approval timing, model-generated workflows, and policy changes during execution. Receipts must identify the policy version, effective corridor, risk basis, and approval basis.
