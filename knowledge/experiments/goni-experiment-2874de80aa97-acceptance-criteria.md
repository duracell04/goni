---
id: GONI-EXPERIMENT-2874DE80AA97
title: Acceptance Criteria
type: experiment
status: draft
implementation_state: not_applicable
proposition: Routine/public-low-risk tasks default local unless they exceed local budgets.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-ROUTE-01-frugal-sovereign-routing.md
  heading: Acceptance Criteria
  revision: d76d9316f094a71209ad5081d17592702325132d
---

# Acceptance Criteria

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Acceptance Criteria

- Routine/public-low-risk tasks default local unless they exceed local budgets.
- Current public research tasks can escalate with public-only or redacted
  payloads.
- Private/sensitive tasks do not send raw context to cloud by default.
- Every model-routing decision that affects output or escalation eligibility has
  an inspectable `llm_route` receipt object.
