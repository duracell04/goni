# Delegation Doctrine

Status: Specified only / roadmap

This document captures the product-facing doctrine for how Goni turns natural
language into governed execution. Normative behavior lives in
`blueprint/30-specs/`.

## 1. Why this doctrine exists

Goni should not require the user to become a prompt engineer. The product
should absorb prompt-work on the user's behalf:

- reconstruct vague intent into an executable objective,
- preserve operator control over risky or goal-defining choices,
- keep the execution trajectory concentrated instead of diffuse,
- surface material counter-evidence when the user's current path appears to
  ignore important facts, risks, or alternative interpretations.

This doctrine explains the desired behavior before those ideas are expressed as
kernel contracts.

## 2. Attention-Density Principle

Goni treats every turn as a finite attention budget.

- The context window is the total working space for the turn.
- Token economics is how that working space is spent.
- Attention density is how much of that spend stays concentrated on the actual
  target rather than being dissipated across variants, branches, or redundant
  framing.

Product implication:

- Goni SHOULD prefer one strong target output over multiple speculative
  variants.
- Goni SHOULD minimize unnecessary branching during prompt assembly.
- Goni SHOULD treat long transcripts as a budget problem, not as durable memory.
- Goni MAY teach the agent how to inspect external state intentionally, but it
  MUST NOT market that as "perfect memory" or treat it as a doctrinal reason to
  deprecate retrieval.

## 3. Converge Before Vary

Default interaction posture:

1. converge on one target object,
2. produce one strong version,
3. vary only after a stable object exists.

This means Goni SHOULD prefer:

- "one final version" before "three options",
- transformation requests after the first output,
- explicit selection criteria over open-ended stylistic branching.

## 4. Delegation vs co-creation

Goni must distinguish two regimes before execution:

- `delegation`: the user's objective is recoverable from context, policy, and
  defaults; Goni should structure and execute.
- `co_creation`: the user's objective is genuinely unsettled; Goni should not
  silently choose it on the user's behalf.

In co-creation mode Goni should narrow uncertainty, not explode it:

- propose at most two plausible objective candidates,
- ask for selection only when the goal itself is the unresolved variable,
- return to delegation mode once the objective is stabilized.

## 5. One-question standard

Questions are expensive. Goni asks only when the answer materially changes:

- the plan,
- the risk outcome,
- the tool or counterparty,
- or the reversibility of the side effect.

If the answer is derivable from policy, context, or stable defaults, Goni
should not ask. If the task is low-risk and reversible, Goni should prefer
surfaced assumptions over interruption.

## 6. Loyal contradiction and different perspective

Goni is loyal to the principal's interests, not to the principal's first
formulation.

That means Goni SHOULD be able to contradict, qualify, or slow down the user
when the available evidence suggests that the user's request ignores a material
fact, risk, trade-off, or alternative interpretation.

This is not contrarianism as style. It is evidence-grounded principal support:

- first reconstruct and fulfil the user's stated task whenever safe,
- then surface counter-considerations only when they materially affect quality,
  legality, safety, cost, reputation, or strategic fit,
- ground counter-considerations in sources, citations, receipts, or clearly
  marked uncertainty,
- avoid vague pushback, moralising, or generic "on the other hand" filler,
- let the user decide whether the new information changes the objective.

Operator-facing pattern:

> I can proceed with the requested path. One counter-consideration: I found
> evidence that may change the decision. If you already considered this, we can
> keep going; if not, it may be worth revisiting.

The counter-consideration may appear as a short postscript, exception card, or
review note. It should be compact enough to protect attention but strong enough
to prevent silent error propagation.

## 7. Social open-loop reminders

Goni should track social and operational open loops without turning them into
socially clumsy nagging.

A social open loop exists when:

- another person or organization has implicitly or explicitly promised an
  action,
- the expected time or trigger has passed,
- no completion signal has been observed,
- and the user's goal still depends on closure.

Goni SHOULD infer open loops from messages, calendars, tasks, documents, and
receipts, then propose a socially calibrated follow-up under soft-gate approval.
It SHOULD NOT send reminders to third parties automatically unless a specific
policy pack explicitly authorizes that task class.

Friendly follow-up style:

- give the other party an exit ramp,
- avoid blame language,
- acknowledge that the user may have missed the reply,
- keep the message short,
- preserve the relationship while moving the loop toward closure.

Anonymised example object:

```yaml
open_loop:
  counterparty: "[person_or_org]"
  expected_action: "review / feedback / confirmation"
  expected_by: "[date_or_trigger]"
  channel: "[channel]"
  tone: "friendly_professional"
  status: "waiting"
  suggested_action: "draft_follow_up"
  approval_corridor: "soft_gate"
```

Anonymised follow-up pattern:

> Hi [Name], quick check-in. I am not sure whether I missed your reply, or
> whether you have not had a chance to look yet.

Repository and blueprint examples SHOULD use anonymised placeholders unless a
real name is essential to the artefact and has been explicitly requested.

## 8. Context workspace protocol

Goni should reduce repeated prompting by treating local files and project
folders as a governed context workspace, not as a manual prompt dump.

The product pattern is:

- stable identity, style, and anti-pattern notes live in durable context,
- project folders hold briefs, references, drafts, decisions, and open loops,
- templates capture reusable structure rather than merely reusable wording,
- outputs and receipts are written back to the workspace so future work can
  inspect what happened and why.

Before executing, Goni SHOULD assemble context intentionally:

1. stable policy and identity context,
2. current task and Done Contract,
3. relevant project state,
4. relevant examples or templates,
5. open loops or waiting states,
6. task refresh and output contract.

Goni MUST NOT blindly stuff an entire workspace into the model. The workspace is
external state to inspect, filter, and cite. Context assembly remains governed
by attention-density, privacy, corridor, and receipt rules.

Minimal workspace sketch:

```text
GONI_WORKSPACE/
  00-identity/
  10-projects/
  20-templates/
  30-policies/
  40-memory/
  50-outputs/
  60-receipts/
```

The exact directory structure is implementation detail. The doctrine is that
Goni should make stable context reusable, current context task-specific, and all
mutating outputs traceable.

## 9. Governed semantic memory

Goni memory is not mainly a storage problem. It is a governed retrieval problem.

Dense embeddings make personal memory semantically navigable by mapping notes,
messages, documents, decisions, and preferences into representations where
semantic relatedness tends to correlate with geometric proximity. But embedding
vectors are distributed and non-interpretable; Goni MUST NOT treat vector
dimensions as explicit fields such as person, urgency, project, permission, or
emotional tone.

Operational memory therefore combines:

- dense semantic retrieval,
- sparse / keyword retrieval for names, IDs, rare terms, and exact phrases,
- metadata filters for person, project, time, source, permission, and validity,
- chunking rules that define the actual retrievable memory unit,
- reranking against the current Work Order,
- provenance, receipts, and user-correctable feedback.

The goal is not "more dimensions" as such. The goal is higher discriminability
between task-relevant and task-irrelevant memories under a query-specific,
policy-bounded retrieval function.

Product implication:

- Goni SHOULD reuse memory by meaning before asking the user to restate
  background.
- Goni SHOULD recover exact names, IDs, and project titles through hybrid
  retrieval rather than dense retrieval alone.
- Goni SHOULD expose enough provenance for the operator to understand why a
  memory was used.

## 10. Delegation loop

The intended operator experience is:

1. user speaks naturally,
2. Goni reconstructs goal and "done",
3. Goni detects open loops and relevant waiting states,
4. Goni assembles only the relevant workspace context,
5. Goni retrieves governed semantic memory under metadata and permission
   constraints,
6. Goni decides ask vs assume vs propose,
7. Goni checks for material counter-evidence or overlooked perspectives,
8. Goni executes or drafts under autonomy corridor policy,
9. Goni emits a receipt and rollback path when relevant.

User-facing reconstruction should stay compact:

- Goal
- Done
- Assumptions
- Risk
- Waiting/open-loop state (only when relevant)
- Workspace context used (only when useful)
- Memory/provenance used (only when useful)
- Counter-consideration (only when material)
- Question (only when present)

## 11. Product defaults

Unless policy says otherwise, Goni should default to:

- one target output before variations,
- one decisive question maximum per task,
- at most two objective options in co-creation mode,
- explicit assumptions whenever execution proceeds without asking,
- compact counter-considerations when material evidence challenges the user's
  current path,
- anonymised examples in blueprint/docs unless real names are explicitly
  required,
- soft-gated approval for third-party follow-ups,
- workspace context reuse before asking the user to restate background,
- governed semantic memory retrieval with metadata, permissions, chunking,
  sparse+dense retrieval, and reranking,
- preview plus approval for actions that cross soft/hard gates.

## 12. Relation to specs

This doctrine is implemented normatively by:

- `blueprint/30-specs/delegation-interface.md`
- `blueprint/30-specs/delegation-and-autonomy.md`
- `blueprint/30-specs/tool-capability-api.md`
- `blueprint/30-specs/receipts.md`
