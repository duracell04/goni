---
id: OBL-01
title: Preventive Obligation Management
type: principle
status: draft
implementation_state: specified_only
proposition: "Goni should evaluate elimination, prevention, consolidation, simplification, and authorized execution of recurring personal obligations to reduce long-run maintenance burden while preserving valuable functions, user preferences, and delegated-authority constraints."
domains:
- product
- agent
- system
aliases: []
relations:
- type: refines
  target: GONI-THESIS-468CFFD9870E
- type: depends_on
  target: DELEG-01
sources: []
artifacts: []
uncertainty: "Avoided future obligations are counterfactual and often only estimable; hidden functions, long-run consequences, and the principal's preferences may be ambiguous. This is a specified product principle, not a proven optimization or a new runtime capability."
legacy: []
---

# OBL-01 — Preventive Obligation Management

## Principle

The objective of personal delegation is to preserve the principal's valued
outcomes with the smallest justified lifetime maintenance burden. A completed
chore is not necessarily an improvement if its underlying cause continues to
generate avoidable work.

Goni should examine source-level prevention and simplification before investing
in repeated task execution. The preferred intervention is the one with the
highest expected net value under the principal's mandate, not automatically the
one that produces the fewest tasks or the most automation.

## Function before form

A personal obligation is an activity, commitment, or decision that consumes
attention or resources over time. It may be necessary, avoidable, intrinsically
valuable, or serve several functions at once. Its apparent task description
may omit less visible functions such as trust, coordination, safety,
relationships, legal compliance, or exception detection.

Before recommending removal, Goni should identify the obligation's source,
recurrence, beneficiary, required outcome, hidden functions, failure costs,
reversibility, and applicable permissions. Removing a visible task is useful
only when the relevant underlying functions remain satisfied.

## Candidate interventions

For a recurring obligation, compare authorized alternatives:

1. **Preserve:** retain the activity when its direct or indirect value warrants it.
2. **Eliminate or prevent:** remove an unnecessary commitment or stop a recurring
   source of avoidable work.
3. **Consolidate or simplify:** reduce duplicate services, handoffs, decisions,
   notifications, or workflow steps.
4. **Automate or delegate:** execute the remaining necessary work using existing
   Work Orders, mandates, corridors, mediation, and receipts.
5. **Defer or escalate:** preserve uncertainty when the expected value of more
   information exceeds the cost or risk of acting.

Examples include proposing to unsubscribe from redundant promotional messages
instead of sorting them forever; consolidating duplicated billing workflows
instead of repeatedly reconciling them; and preserving valuable relationship
follow-ups even when they require effort.

Goni may generate and rank such proposals as cognition. Cancellation, changes
to subscriptions or commitments, external communications, and any other
consequential effects remain subject to existing policy, approval,
capability-scoped execution, and receipt requirements. Preference inference is
not authorization.

## Analytical model

Let B_t represent outstanding personal obligations, A_t(π) newly arising
obligations under policy π, and D_t(π) obligations completed or resolved:

$$
B_{t+1} = \max\{0,\;B_t + A_t(\pi) - D_t(\pi)\}.
$$

The important new control variable is A_t(π): suitable upstream
interventions can reduce future avoidable arrivals, whereas completion-only
delegation primarily increases D_t(π). This personal-obligation process is
distinct from Goni's computational job queues.

For an evaluation horizon T, choose among **authorized** policies using the
principal's expected net value:

$$
\pi^* \in \arg\max_{\pi\in\Pi_{\mathrm{authorized}}}
\mathbb{E}\left[\sum_{t=0}^{T}
\left(V_{\mathrm{preserved},t}
-C_{\mathrm{human},t}
-C_{\mathrm{coordination},t}
-C_{\mathrm{oversight},t}
-C_{\mathrm{financial},t}
-C_{\mathrm{risk},t}\right)\right].
$$

Terms require domain-appropriate measurement; apparent workload reduction
must not be scored as success when it destroys valuable outcomes or transfers
greater costs elsewhere. This is a queueing and decision-theoretic model, not
a claim that household disorder is thermodynamic entropy or enthalpy.

## Architecture and scope

OBL-01 refines Goni's existing product objective. It adds a **selection
criterion for proposed Work Orders**, not a fifth system plane, independent
memory store, new authority class, or separate scheduler. Existing Observation
→ Reflection → Planning may identify repetitive patterns and propose upstream
interventions; existing DELEG-01 determines whether an intervention may
become an external effect.

Initial scope is recurring digital-life administration (communications,
calendar, subscriptions, documents, and workflows). Physical chores or
embodied action require their own applicable capabilities and permissions.

## Falsifiability

OBL-EVAL-01 compares prevention-aware policies with completion-only
delegation over matched, longitudinal scenarios. A net reduction in
maintenance costs is useful only if required outcomes, user agency, and
authority invariants remain intact.
