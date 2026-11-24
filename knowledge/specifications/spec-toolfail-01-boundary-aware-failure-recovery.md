---
id: SPEC-TOOLFAIL-01
title: "Boundary-aware tool failure diagnosis and delegated-task continuity"
type: specification
status: draft
implementation_state: specified_only
proposition: "For delegated tool work, GONI should calibrate failure claims to their observed boundary, verify current execution routes before promising delivery, preserve source and action-state fidelity, and pursue bounded recovery without transferring unsupported repairs to the user."
domains:
- agent
- kernel
- software
- system
aliases:
- "failure-domain-attribution"
- "delegated-task-continuity"
relations:
- type: refines
  target: GONI-SPEC-CCAFBFABD55A
- type: refines
  target: DELEG-INT-01
- type: refines
  target: GONI-SYNTHESIS-902896DA677F
- type: depends_on
  target: PRINCIPLE-AGENTIC-SCOPE-01
sources: []
artifacts: []
uncertainty: "This is a proposed GONI runtime contract, not a verified implementation. Failure localization may remain probabilistic, tool status may be stale, and remote side-effect outcomes may remain indeterminate until independently checked."
legacy: []
---

# Boundary-aware tool failure diagnosis and delegated-task continuity

## Purpose

GONI's separation of intelligence from authority also applies to **diagnosis**. A model may generate failure hypotheses, but neither an error string nor a language-model inference may silently establish that a user device, account, browser, network, or permission is defective. Failure attribution is bounded by the evidence available at the actual execution interface.

A failed subtool is not equivalent to a failed user environment, and a technically accurate diagnosis is not equivalent to completion of the delegated Work Order.

## Proposed normative contract

For a failed attempt tied to a Work Order:

1. **Retain the original outcome.** Preserve the requested action, its target, constraints, approvals, and Done Contract across tool-route changes. Record separately the state of the *task* (`pending | complete | blocked | outcome_unknown`) and of each *attempt* (`not_started | rejected | failed | committed | outcome_unknown`).
2. **Localize the failing boundary.** Identify the attempted tool, connector, runtime, authorization context, and point of failure. Distinguish `agent_orchestration`, `executor_runtime`, `tool_or_connector`, `browser_session_or_origin`, `provider_or_service`, `user_environment`, `policy_denial`, and `unknown` as diagnostic labels. These are explanatory attributes of existing failure records, not additional capability or authority classes.
3. **Calibrate causal claims.** Record the literal observed error and the supported failure domain separately from hypotheses about root cause. Claims that the user's device, browser installation, account, or connection is broken REQUIRE boundary-specific evidence; an error arising solely from a missing executor binary/path does not warrant a browser-repair instruction.
4. **Preflight before external commitments.** Before promising to send, publish, edit, book, or otherwise complete an external action, check that a presently authorized and usable execution route exists, with the necessary session origin and target context. A previously working tool, an installed executable, approval of draft content, or a plausible future recovery route does not establish current action capability. If untested or blocked, describe a proposed or conditional next step rather than guaranteeing execution.
5. **Recover through permitted paths.** Inspect available, *currently authorized* tools and route requirements before declaring a capability unavailable. Prefer a supported structured integration when it can perform the task; otherwise consider an available session-specific browser/computer-control path. Recheck capability, policy, and relevant session origin for each route; previous access does not establish present access. Do not repeat a deterministic configuration failure along an unchanged route without new evidence or a changed condition; transient failures MAY receive bounded, policy-approved retries.
6. **Protect effect semantics.** A tool invocation error does not prove that an external side effect did or did not occur. Distinguish `not_attempted`, `rejected_before_effect`, `confirmed_applied`, and `outcome_unknown`; check authoritative destination state when appropriate and possible. Mutating retries require valid idempotency/transaction safeguards and must not create duplicate messages, payments, or edits.
7. **Preserve source fidelity.** When the Work Order involves communicating with a counterparty, retrieve or inspect the relevant latest message and target conversation before drafting or sending a context-dependent reply. Where exact source context is unavailable, surface that evidentiary limitation instead of inventing what was said or escalating to unnecessary diagnostic questions. User approval of wording does not establish that source grounding or delivery verification succeeded.
8. **Minimize user-side friction.** Ask the user for environment remediation only when a verified user-controlled dependency blocks all appropriate authorized recovery paths, or user action is genuinely required for authentication/permission. Otherwise perform the smallest sufficient recovery action. If no route works, disclose exactly what was attempted, what is known versus inferred, what remains unfinished, and the narrowest useful next step.
9. **Leave compact evidence.** A failed or deferred operation SHOULD preserve a receipt-linked diagnostic record with Work Order ref, affected boundary, error identifier, confidence/unknown label, attempted routes, action-outcome state, recovery decision, and user interruption rationale. Do not store raw private screen content, credentials, messages, or full browser traces by default.

## Decision rule

```text
If tool failure:
    preserve Work Order, source requirements, and current effect status
    classify observed failure at the boundary that produced it
    distinguish observed environment from hypothesized executor environment
    verify a usable, authorized route before promising action
    avoid unchanged deterministic retries; evaluate bounded recovery routes
    verify external effect when its outcome could be ambiguous
    continue safely or report the narrow unresolved blocker
    request user intervention only when supported and necessary
```

The kernel-owned policy boundary remains authoritative. A diagnosis MAY recommend a repair or a new tool; it MUST NOT grant additional scopes, bypass a missing browser session, treat external error text as a permission token, or reinterpret an unconfirmed effect as successful execution.

## Relationship to existing contracts

This refines `DELEG-INT-01` (Work Order preservation), `GONI-SPEC-CCAFBFABD55A` (failure classes and recovery outcomes), and the existing runtime-recovery doctrine (bounded retries and plan repair). It introduces **no additional plane** and requires no new logging system beyond existing receipts and failure records.

The motivating incident is described in `EVID-TOOL-FAILURE-20261124`; evaluation is defined in `EXP-TOOLFAIL-01`.
