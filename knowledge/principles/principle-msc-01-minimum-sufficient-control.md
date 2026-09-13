---
id: MSC-01
title: Minimum Sufficient Control Principle
type: principle
status: draft
implementation_state: specified_only
proposition: Goni should apply the least control sufficient to keep expected delegation risk within policy, scaling cognitive effort with task difficulty and governance effort with consequence while preserving autonomy where actions are reversible, verifiable, and bounded.
domains:
- delegation
- governance
- autonomy
- verification
- routing
aliases:
- Adaptive Delegation Optimum
- AI Bliss Point
- smallest sufficient governance
relations:
- type: refines
  target: DELEG-01
  note: Operationalizes bounded delegation as a marginal control-allocation principle.
- type: refines
  target: GONI-DECISION-5A5B3B8A34CA
  note: Makes autonomy-corridor width and escalation explicitly proportional to consequence, reversibility, and verifiability.
- type: refines
  target: GONI-DECISION-84F201170733
  note: Separates cognition allocation by task difficulty from governance allocation by consequence.
sources: []
artifacts: []
uncertainty: Qualitative design principle; control costs, failure probabilities, and policy thresholds are not yet empirically calibrated.
legacy: []
---

# Minimum Sufficient Control Principle

> Status boundary: this is a `draft` / `specified_only` design principle. The optimization framing below defines intended system behavior, not an empirically calibrated policy or observed runtime property.

## Proposition

Goni should apply the least control sufficient to keep expected delegation risk within policy, scaling cognitive effort with task difficulty and governance effort with consequence while preserving autonomy where actions are reversible, verifiable, and bounded.

The principle generalizes the idea of a smallest sufficient specification into **smallest sufficient governance**. More control is valuable while it removes material uncertainty, protects an invariant, or reduces consequential risk enough to justify its cost. Beyond that point, extra specification, context, verification, approval, or architectural prescription can add friction, conflict, rigidity, or interference without improving the outcome enough to justify the burden.

## Conceptual optimization

Let `a` denote useful execution autonomy and `c` denote control intensity. A conceptual objective is:

\[
(a^*, c^*) = \arg\max_{a,c}\left[V(a) - C_{exec}(a) - C_{control}(c) - P_{fail}(a,c)L_{fail}\right]
\]

This is a design heuristic rather than a calibrated utility function. Goni should increase control while the expected marginal reduction in failure loss exceeds the marginal cost of that control.

The resulting optimum is task-dependent. There is no universal trust level or fixed amount of supervision that is correct for every action.

## Separate cognition from governance

Two allocations must remain distinct:

- **Cognitive effort scales primarily with difficulty.** Constraint coupling, ambiguity, horizon length, exactness requirements, and task novelty can justify more context, stronger models, deeper reasoning, tool-assisted analysis, or additional feedback loops.
- **Governance effort scales primarily with consequence.** High loss, irreversibility, weak verifiability, large blast radius, external side effects, or legal and financial significance justify narrower capability corridors, stronger verification, approval gates, and rollback requirements.

A task may therefore be intellectually difficult but operationally harmless, or cognitively trivial but highly consequential. Greater reasoning allocation must not imply greater execution authority.

## Control-allocation signals

| Signal | Preferred response |
| --- | --- |
| Higher task difficulty, coupling, novelty, or exactness | Increase cognitive effort and verification targeted at correctness. |
| Higher consequence, irreversibility, ambiguity, or external side effects | Increase governance intensity and narrow authority. |
| Higher reversibility, testability, observability, and bounded blast radius | Permit broader autonomous execution within policy. |
| Low-information or redundant instruction | Preserve model discretion instead of adding control. |
| Cheap, reliable automated checks | Prefer closed-loop verification over prospective micromanagement. |

This preserves a central distinction: **autonomy is not authority**. Goni can grant high execution autonomy inside a tightly bounded authority corridor.

## Minimum sufficient specification and context

Prompt detail, context, scope, constraints, tool access, and reasoning effort are control surfaces rather than ends in themselves. Goni should add them when they resolve a concrete ambiguity, protect a hard invariant, or materially increase expected outcome quality.

Additional specification can exhibit diminishing and, under interference, negative marginal returns. Harm is most likely when new instructions introduce contradiction, irrelevant context, premature architecture, constraint coupling, proxy optimization, or unnecessary reduction of useful degrees of freedom. Long specifications remain appropriate when the task genuinely requires them.

For consequential multi-step work, feedback can substitute for large amounts of prospective instruction:

`understand -> act -> measure -> correct -> verify`

The preferred execution unit is one coherent consequential objective that can be completed, checked, and either accepted or rolled back as a closed loop.

## Proportional verification

Verification should be economically proportional to expected error loss. A useful heuristic is:

\[
EV(verification) \approx P(error) \times L(error) \times P(detection \mid verification)
\]

Goni should add verification while its expected benefit exceeds its marginal cost. Cheap, reversible, highly observable actions can tolerate lightweight checks; expensive, hidden, irreversible, or legally consequential effects require stronger independent evidence or human authorization.

## System implications

A future implementation of this principle can let the orchestrator, router, kernel, and policy layer allocate several controls independently:

- specification and prompt detail;
- context and retrieval budget;
- model class and reasoning budget;
- scope and execution horizon;
- tool and capability scope;
- autonomy-corridor width;
- verification depth and evaluator independence;
- human approval thresholds;
- rollback requirements; and
- receipt and evidence depth.

The orchestrator can compile different task contracts for different model and tool configurations while preserving one model-independent objective and one kernel-owned authority boundary. Stronger agents can receive broader solution-space freedom when verification is adequate; weaker, smaller, or locally constrained agents can receive narrower work units and more frequent deterministic checkpoints. Model capability changes the execution scaffold, not the authority granted to it.

This turns the principle into an adaptive delegation policy rather than a fixed prompt template. The kernel can preserve maximum useful delegation while policy remains the authority boundary.

## Boundary conditions

This principle does **not** claim a universal inverted-U relationship between control and performance. Additional control may simply reach diminishing returns; it becomes counterproductive when it creates interference, conflict, rigidity, or disproportionate cost.

The principle also does not authorize broader corridors by itself. Existing mandates, policy, capability scopes, and explicit approval requirements remain authoritative. `MSC-01` governs how much control to allocate *within* those boundaries.

## Operational shorthand

> Spend instructions where uncertainty is expensive. Spend verification where errors are expensive. Preserve freedom where intelligent judgment has value.
