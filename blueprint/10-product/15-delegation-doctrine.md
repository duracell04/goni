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

## 11. Change-velocity gradient

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

## 12. Harness Governance

Goni's intelligence is not only in the model. It is in the governed harness
that decides what the model sees, may do, must verify, can remember, and must
undo.

The **Goni Harness Plane** is the product-facing name for this governed
operating layer. It is not a separate hidden agent framework. It is the
versioned control overlay across context assembly, memory retrieval, tool
permissions, routing, approval corridors, receipts, evaluation, and rollback.

Harness artefacts SHOULD be explicit, inspectable, editable, auditable, and
reversible. At minimum, this includes:

- delegation doctrine and Work Order / Done Contract templates,
- memory retrieval and context assembly policies,
- model routing and model-installation policies,
- approval corridors and social open-loop rules,
- tool capability manifests and tool-use templates,
- prompt and context assembly templates,
- receipt formats and evaluation packs.

Harness improvement MUST NOT be treated as silent prompt drift. A meaningful
harness change should state what it expects to improve, what evidence will
measure the change, the evaluation window, and the rollback condition. This
turns operating-procedure changes into falsifiable artefacts rather than
folklore.

Goni therefore learns operating procedures, not only memories:

- which workflow worked,
- which retrieval strategy missed evidence,
- which clarification was unnecessary,
- which model route was sufficient,
- which approval corridor caused friction,
- which reminder tone was approved,
- which policy or tool path required rollback.

Receipts remain the source evidence. Higher-level experience digests and
harness-change manifests are derived artefacts that summarize approval, edit,
override, rejection, retrieval, routing, latency, cost, policy, and rollback
signals without feeding raw logs back into the model blindly.

This doctrine follows the ML-systems lesson that non-model infrastructure can
create hidden debt if it is treated as glue, and the agentic-systems lesson that
harness design directly shapes task competence [[sculley2015-hidden-tech-debt]]
[[lin2026-agentic-harness-engineering]]. Goni's local-first stance makes the
harness user-owned and reversible rather than silently vendor-managed.

## 13. Product defaults

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
- preview plus approval for actions that cross soft/hard gates,
- fast iteration at the edge and evidence-backed promotion toward the core.

## 14. Frugal Sovereign Routing

Goni applies FrugalGPT-style cascades under sovereignty constraints. The default
route is not the strongest model or the cloud Council. The default route is the
least external, least costly, sufficiently capable path.

Routing order:

```text
rule / cache / memory answer
-> small local model
-> larger local model
-> specialist local tool / RAG / corpus reading
-> local multi-agent check
-> cloud LLM Council
-> premium cloud model / multi-model vote
```

The router SHOULD optimize across quality, latency, cloud cost, privacy leakage
risk, energy/thermal cost, audit burden, external dependency cost, data
locality, and the active escalation corridor.

Council escalation SHOULD occur only when one of these predicates is true:

- current public web or broader external model knowledge is required,
- local confidence or local evidence is insufficient,
- model disagreement checking is useful for high-stakes work,
- local compute would exceed the configured time or thermal budget,
- the user explicitly requests external reasoning,
- policy authorizes a redacted or public-only cloud payload.

Raw private, legal/financial/identity-sensitive, or confidential context MUST
NOT be sent to cloud by default. It requires redaction, a policy corridor, or
explicit approval. The routing decision itself is receipted so cloud escalation
is accountable instead of implicit.

One-line doctrine:

> FrugalGPT asks which model is cheapest for the task; Goni asks which route is
> sufficient, private, auditable, and local enough.

## 15. Relation to specs

This doctrine is implemented normatively by:

- `blueprint/30-specs/memory-retrieval.md`
- `blueprint/30-specs/delegation-interface.md`
- `blueprint/30-specs/delegation-and-autonomy.md`
- `blueprint/30-specs/tool-capability-api.md`
- `blueprint/30-specs/receipts.md`
