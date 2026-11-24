---
id: EXP-AGENTIC-SCOPE-01
title: "Scope proportionality and marginal-value stopping evaluation"
type: experiment
status: draft
implementation_state: not_applicable
proposition: "A matched evaluation should test whether scope and stopping controls reduce unnecessary interventions and lifecycle liabilities without degrading verified completion, authority compliance, or safety."
domains:
- agent
- evaluation
- governance
aliases: []
relations:
- type: tests
  target: PRINCIPLE-AGENTIC-SCOPE-01
- type: depends_on
  target: DELEG-INT-01
sources: []
artifacts: []
uncertainty: "Protocol only. No benchmark results, effect sizes, thresholds, incident causation, or runtime implementation are established. Expert assessment of avoidable work and lifecycle cost is fallible and must be documented."
legacy: []
---

# Scope proportionality and marginal-value stopping evaluation

**Status:** Proposed experiment; results and GONI implementation evidence are not yet available.

## Research question

Does adding objective-preservation, early feasibility checks, explicit scope gates, and marginal-value stopping rules improve **whole-system delegated utility**, compared with an otherwise identical agent/harness, while maintaining task quality and authority boundaries?

## Matched protocol

Use paired, representative Work Orders and immutable Done Contracts across small document, coding, inbox, research, and integration tasks. Compare baseline and scoped-control variants with the **same** model family, context, tools, permissions, compute/time budgets, and starting artifacts. Include feasible tasks, deliberately infeasible dependencies, attractive but unnecessary extensions, and tasks where additional safety work is genuinely required. Randomize order and independently review completed artifacts and receipts.

At predetermined milestones, record whether a critical feasibility assumption was tested; whether a new dependency, service, permission, or architectural commitment was proposed and authorized; why each material action was considered necessary; and why the run stopped or escalated. Label avoidable work using blinded expert review against the principal's original Done Contract, documenting disagreements.

## Evaluation endpoints

- **Primary:** verified Done Contract completion conditional on policy compliance, together with total user-valued burden (time, resource cost, review effort, integration and estimated lifecycle liabilities).
- **Scope:** rate and materiality of unrequested expansion; avoidable dependencies and interventions; unauthorized material changes.
- **Feasibility:** time/resources spent before decisive assumptions are checked; work lost after an invalidating finding.
- **Stopping:** excess actions after sufficiency, premature termination rate, and unnecessary continuation when estimated marginal benefit is below cost.
- **Safety and agency:** unauthorized effects, irreversibility/rollback burden, review interruptions, and whether assumption, cost, and escalation records can be reconstructed.

Report paired task-level distributions, uncertainty intervals, error examples, and safety/quality trade-offs. A putative benefit is falsified by reduced accepted task completion, more unauthorized effects, substantial false early stopping, or increased system-level cost despite fewer steps. Numeric thresholds and acceptance criteria must be prespecified before testing.

## Evidence boundary

This node specifies **what to measure**, not that the intervention works. Trace-level evidence can show what occurred; causal attribution requires a controlled comparison and assessment of rival explanations. Do not equate local technical success, agent activity, or test counts with principal utility.
