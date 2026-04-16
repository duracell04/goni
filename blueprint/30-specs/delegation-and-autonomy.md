---
id: DELEG-01
type: SPEC
status: specified_only
---
# DELEG-01 - Delegation and Autonomy
DOC-ID: DELEG-01
Status: Specified only / roadmap

This spec defines how Goni maximizes safe background execution of digital work
while preserving policy-level human control.

Delegation in Goni is not prompt relay. The system is expected to perform part
of the prompt-work on behalf of the user: infer missing structure, repair vague
intent into executable form, and acquire just enough extra context to act
safely. Those interpretive moves must remain visible, corrigible, and bounded
by policy rather than hidden inside model behavior
[[tomasev2026-intelligent-delegation]] [[zhang2025-ace]]
[[yang2025-contextagent]].

## 1. Purpose

Goni is designed for policy-and-anomaly-first operation:

- users define constraints and priorities,
- the twin executes routine work end-to-end within those constraints,
- users review exceptions and policy drift, not every action.

This contract applies across mail, calendar, documents, payments, and other
tool-mediated surfaces.

## 2. Delegation model

### 2.1 Task classes

Every mediated action MUST be tagged with a `task_class` (for example:
`email_reply`, `invoice_payment`, `calendar_change`, `doc_edit`).

### 2.2 Delegation-engineering stack

Delegated execution MUST expose four cooperating layers:

- `behavioral`: concise, structured output and stable control language.
- `dialogic`: ask only decisive clarification questions.
- `epistemic`: surface assumptions, uncertainty, and missing context.
- `executive`: convert intent into plan, tool proposals, and mediated actions.

These layers are logical behaviors, not separate models. They exist so the
operator can inspect where an execution decision came from and correct the
right layer when delegation fails.

### 2.3 Operator contract

For delegable work, the runtime MUST follow this operator contract:

- infer missing structure from policy, prior context, and task class before
  interrupting the user,
- ask a clarification question only when the answer materially changes risk,
  corridor, tool choice, or irreversible side effects,
- surface assumptions and uncertainty when proceeding without clarification,
- convert repaired intent into a bounded plan and explicit tool intent before
  any mutating call.

This treats delegation as mixed-initiative control under uncertainty rather
than as literal prompt completion [[horvitz1999-mixed-initiative]]
[[tomasev2026-intelligent-delegation]].

### 2.4 Autonomy corridors

Each `task_class` MUST declare one of:

- `no_go`: never execute automatically.
- `soft_gate`: execute only with bounded conditions and review gates.
- `autopilot`: execute by default without pre-approval.

These are the three kernel authority corridors. Corridor assignment is
policy data, versioned, and owned by the Goni kernel rather than by external
assistant frameworks or gateways.

For operator-facing discussion, Goni may describe these as:

- `autopilot`: execute inside the active risk threshold,
- `soft_gate`: require lightweight approval or bounded review,
- `hard_gate`: require explicit human decision.

`no_go` remains the deny-only policy floor in the normative corridor schema.

### 2.5 Risk dimensions

A normalized `risk_score` in `[0,1]` MUST be computed from, at minimum:

- reversibility and compensation path quality,
- blast radius (financial, legal, or reputational magnitude),
- ambiguity/uncertainty,
- policy sensitivity (regulated domain, restricted counterparty, or consent gate).

## 3. Default execution policy

Goni uses an "auto unless risky" policy:

- if corridor is `autopilot` and `risk_score <= theta_auto`, execute;
- if corridor is `soft_gate` and `risk_score <= theta_soft`, execute with
  queued review;
- otherwise escalate to user decision.

`theta_auto` and `theta_soft` are explicit policy parameters and must be auditable.

### 3.1 Clarification policy

Clarification is a bounded interrupt class, not a default interaction style.

- The runtime MAY ask a decisive clarification question when missing
  information would materially change `risk_score`, `task_class`,
  `autonomy_mode`, or the legality/reversibility of a side effect.
- The runtime MUST NOT ask questions that can be answered from active policy,
  retrieved context, prior approvals, or deterministic task constraints.
- If clarification budget is exhausted, deferred by policy, or not worth the
  interruption cost, the runtime MUST either:
  - proceed with surfaced assumptions inside the active corridor, or
  - escalate/block the action if safe execution is not possible.

Any clarification decision must be auditable through receipts and scheduler
events.

### 3.2 Control-plane contract

Delegation is policy-first:

- the model proposes intent repair, plans, and tool actions,
- the kernel authorizes or denies execution,
- irreversible side effects require explicit approval or an approved two-phase
  commit path,
- every side effect emits a receipt with delegation metadata.

## 4. SOP lifecycle

Standard operating procedures (SOPs) are machine-executable templates.
Each SOP MUST move through:

1. `shadow` (suggestions only, no external side effects),
2. `approved` (user approved),
3. `autopilot` (eligible for automatic execution),
4. `revoked` (disabled and retained for audit).

Promotion to `autopilot` requires successful runs and no unresolved safety
findings over a configured observation window.

## 5. Oversight model

Oversight is post-hoc by default:

- anomaly-first feed (only blocked, high-risk, or policy-drift events),
- periodic sampling of autonomous actions,
- fast policy override and corridor downgrade.

This avoids per-action approval loops while preserving meaningful human control.

## 6. Failure modes

Delegation quality MUST be evaluated against failure modes, not only task
success:

- `lazy_agent`: asks the user for structure it could have inferred locally.
- `overcautious_agent`: escalates or blocks routine work that fits active
  policy.
- `shape_shifter`: changes plan or rationale without surfacing the update.
- `complacency_engine`: proceeds confidently despite unresolved ambiguity.
- `hidden_assumption_executor`: makes materially important assumptions without
  exposing them.

Policies, evals, and replay traces should be able to distinguish these modes so
fixes can attach to the right control seam.

## 7. Autonomy packs and templates

Goni SHOULD support profile-based policy packs and SOP templates so users can
start with safe defaults and adapt over time.

Each imported pack MUST declare:

- covered task classes,
- default corridors and thresholds,
- declared no-go actions,
- provenance (pack version, author, policy hash).

Packs MAY also declare delegation-policy defaults for clarification thresholds,
assumption visibility, and irreversible-action rules.

## 8. Invariants

- **I1 - Policy primacy:** no autonomous execution without an explicit corridor
  and policy hash.
- **I2 - Risk-bounded autonomy:** autonomous execution requires computed risk
  below active thresholds.
- **I3 - Auditable delegation:** every autonomous or escalated action emits a
  receipt with autonomy and risk fields.
- **I4 - Fail closed:** if risk computation, policy load, or capability
  validation fails, execution is denied and logged.
- **I4a - Kernel-owned corridors:** authority corridors are defined and applied
  by kernel policy, not by third-party session or gateway logic.
- **I5 - Visible intent repair:** mutating execution requires an auditable chain
  from repaired intent to plan to tool intent.
- **I6 - Decisive questioning only:** clarification interrupts are allowed only
  when they materially change safe execution or policy outcome.
- **I7 - Surfaced assumptions:** proceeding under ambiguity requires explicit
  assumption and uncertainty metadata.

## 9. Related specs

- [Tool capability API](/blueprint/30-specs/tool-capability-api.md)
- [Scheduler and interrupts](/blueprint/30-specs/scheduler-and-interrupts.md)
- [Receipts](/blueprint/30-specs/receipts.md)
- [Network gate and anonymity](/blueprint/30-specs/network-gate-and-anonymity.md)

## 10. Upstream

- [Software requirements](/blueprint/software/10-requirements.md)
- [Software decisions](/blueprint/software/90-decisions.md)

## 11. Downstream

- [Conformance](/blueprint/software/30-conformance.md)
- [Eval lane](/blueprint/50-evidence/eval/README.md)

## 12. Adjacent

- [Metrics](/blueprint/docs/metrics.md)
- [Bibliography](/blueprint/docs/references/bibliography.md)

## Conformance tests

- mutating delegated actions must have an auditable chain:
  `intent_summary -> plan_summary -> tool_intent`
- clarification interrupts must occur only when a missing answer would change
  corridor, risk, or irreversible behavior
- actions that proceed without clarification must surface assumptions and
  uncertainty in receipts
- irreversible actions must require explicit approval or a declared two-phase
  commit path
- failure-replay suites must classify at least the documented delegation
  failure modes
