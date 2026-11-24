---
id: PRINCIPLE-AGENTIC-SCOPE-01
title: "Objective-preserving scope proportionality"
type: principle
status: draft
implementation_state: specified_only
proposition: "Delegated work should preserve the principal's accepted outcome and stop at the smallest sufficient intervention, with early decisive feasibility checks and marginal-value scrutiny of material scope expansion."
domains:
- agent
- governance
- system
aliases:
- "minimal-sufficient-intervention"
relations:
- type: refines
  target: DELEG-INT-01
- type: refines
  target: DELEG-01
- type: depends_on
  target: GONI-THESIS-7BFB74017D50
- type: supports
  target: GONI-SYNTHESIS-E93AC4B42387
sources:
- SRC-SIMON1955-RATIONAL-CHOICE
- SRC-HOLMSTROM-MILGROM1991-MULTITASK
- SRC-MANHEIM-GARRABRANT2019-GOODHART
- SRC-SCULLEY2015-HIDDEN-TECH-DEBT
artifacts: []
uncertainty: "This is a proposed normative design contract, not observed GONI enforcement. Marginal value and lifetime cost are uncertain estimates. The cited literatures offer explanatory lenses rather than verified causal diagnoses of specific agent runs; effects require matched evaluation."
legacy: []
---

# Objective-preserving scope proportionality

## Proposition

A technically competent autonomous agent may complete intermediate tasks correctly yet fail its principal through cumulative scope expansion. Additional components, refinements, integrations, and verification tasks can be locally defensible while reducing the net value of the originally authorized outcome. GONI therefore treats **scope proportionality** as a delegation-governance constraint: greater reasoning capacity should ordinarily identify a sufficient intervention with less unnecessary work, rather than enlarge the intervention by default.

## Theoretical grounding and limits

- **Bounded rationality and satisficing.** Simon (1955) motivates an acceptance threshold under costly search: a satisfactory, evidenced result can rationally terminate further exploration.
- **Multitask principal–agent problems.** Holmström and Milgrom (1991) show how attention to measurable components can diverge from a principal's broader objective. For GONI this is an analytical analogy about delegated specifications and incentives, **not** evidence that models possess independent human-like motives.
- **Goodhart-style proxy optimization.** Manheim and Garrabrant (2019) distinguish ways that increased optimization pressure on imperfect metrics can undermine the intended goal. Code volume, test counts, architectural sophistication, or the number of resolved subtasks are candidate progress signals, never substitutes for user-valued completion.
- **System-level technical debt.** Sculley et al. (2015) document how successful local ML changes can create hidden dependencies, entanglement, and maintenance liabilities.

These frameworks identify **plausible mechanisms**, not a demonstrated causal explanation of any particular agent episode. Diagnosis requires the original Work Order, execution record, permission state, sequence of changes, and independently assessed outcomes.

## Proposed control-plane contract

1. **Bind the objective.** Every consequential delegation starts from an accepted Work Order and Done Contract stating the intended outcome, quality and safety floors, material constraints, budget, and authorized action surface. Model-generated subgoals remain proposals; technical usefulness does not by itself authorize new scope.
2. **Test decisive assumptions early.** Before expensive or persistent implementation choices, identify the feasibility assumptions whose failure would invalidate the proposed path. Prefer inexpensive, reversible validation of the highest-consequence uncertainty.
3. **Evaluate cumulative effects.** At material plan changes, compare prospective incremental benefit to total incremental cost: compute and money, user attention, implementation effort, integration and maintenance liabilities, security exposure, reversibility, lock-in, and opportunity cost. Judge local improvements by their effect on the entire approved outcome.
4. **Gate scope expansion.** New external services, privileged access, durable infrastructure, recurring expense, providers, or significant architectural dependencies require their own kernel policy/mandate decision, even if they would improve a subtask. The agent MAY recommend these alternatives with reasons; it MUST NOT turn them into implicit authority.
5. **Terminate or escalate.** Once the Done Contract is verified, stop unless the principal authorizes an additional objective. Earlier, continue an optional intervention only where its evidence-calibrated expected benefit exceeds its full expected marginal cost while remaining inside policy and quality constraints. If a critical assumption fails, a budget is exhausted, or the remaining path requires ungranted authority, stop, repair within scope, or seek a decisive decision.
6. **Leave decision evidence.** Material continuation, expansion, termination, and escalation decisions SHOULD be reconstructable through existing Work Orders, policy decisions, evaluation artifacts, and receipts. Record the decisive assumption or marginal-value basis proportionately; routine micro-operations need no new bureaucratic workflow.

The marginal-value comparison is a **decision heuristic**, not a proof that utility can be measured precisely:

```text
Continue a material optional action only if:
  authorized(action)
  AND required quality/safety floors remain satisfiable
  AND E[incremental value to Done Contract | evidence]
        > E[total incremental lifecycle cost | evidence].
```

The optimal intervention is the **least burdensome sufficient intervention**, not mechanically the fewest tool calls or the shortest implementation. Additional verification, redundancy, or security controls are justified when they materially improve the accepted result or satisfy non-negotiable constraints.

## Falsification

A scope policy fails when it prematurely terminates needed work, lowers verified task success or safety, causes more costly user interventions, or admits material dependencies without authorization. Its benefits remain unproven until tested against representative tasks and a comparable baseline; see `EXP-AGENTIC-SCOPE-01`.
