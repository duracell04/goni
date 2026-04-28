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
- keep the execution trajectory concentrated instead of diffuse.

This doctrine explains the desired behavior before those ideas are expressed as
kernel contracts.

The same rule applies to memory. Goni should not ask the user to decide whether
something is a saved memory, project note, vector embedding, tag, folder, or
reminder. The product-facing promise is that Goni turns personal memory into
governed system work: classify, chunk, index, retrieve, filter, act, and write
receipts without making the user manage memory architecture.

The defensible claim is architectural, not empirical overclaim. Goni is designed
to be better suited for personal AI memory because it treats memory as a
local-first governed retrieval plane rather than a generic product feature.
Evaluation must still prove task outcomes, retrieval quality, safety, and user
benefit.

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

## 6. Delegation loop

The intended operator experience is:

1. user speaks naturally,
2. Goni reconstructs goal and "done",
3. Goni decides ask vs assume vs propose,
4. Goni executes under autonomy corridor policy,
5. Goni emits a receipt and rollback path when relevant.

User-facing reconstruction should stay compact:

- Goal
- Done
- Assumptions
- Risk
- Question (only when present)

## 7. Change-velocity gradient

Goni should be fluid at the edges and constitutional at the core.

Change velocity decreases as governance centrality increases. Components closest
to authority, identity, memory integrity, privacy, and irreversible action move
slowest; components farther from the core may adapt, branch, and experiment
quickly.

Product implication:

- Prompts, drafts, retrieved context, one-off plans, model candidates, plugins,
  and experimental retrieval strategies MAY move quickly.
- Project memory, user preferences, relationship patterns, templates, and
  recurring workflows SHOULD update often, but with source, timestamp,
  permission, and correction history.
- Control-plane policy, Work Orders, model permission rules, memory access
  classes, and tool-use corridors MUST change through versioned policy,
  receipts, tests, and rollback.
- Kernel invariants, privacy posture, capability boundaries, receipt
  requirements, and local-first sovereignty assumptions MUST change rarely and
  only through evidence-backed governance.

This is the difference between adaptivity and drift: experiments happen outside
the core; promotion inward requires evidence.

## 8. Product defaults

Unless policy says otherwise, Goni should default to:

- one target output before variations,
- one decisive question maximum per task,
- at most two objective options in co-creation mode,
- explicit assumptions whenever execution proceeds without asking,
- preview plus approval for actions that cross soft/hard gates,
- fast iteration at the edge and evidence-backed promotion toward the core.

## 9. Relation to specs

This doctrine is implemented normatively by:

- `blueprint/30-specs/memory-retrieval.md`
- `blueprint/30-specs/delegation-interface.md`
- `blueprint/30-specs/delegation-and-autonomy.md`
- `blueprint/30-specs/tool-capability-api.md`
- `blueprint/30-specs/receipts.md`
