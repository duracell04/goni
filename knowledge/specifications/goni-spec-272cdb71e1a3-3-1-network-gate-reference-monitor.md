---
id: GONI-SPEC-272CDB71E1A3
title: 3.1 Network Gate (reference monitor)
type: specification
status: draft
implementation_state: specified_only
proposition: 'The Network Gate is both: Policy Decision Point (PDP): decides allow/deny and route selection.'
domains:
- network
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/network-gate-and-anonymity.md
  heading: 3.1 Network Gate (reference monitor)
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 3.1 Network Gate (reference monitor)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.1 Network Gate (reference monitor)

The Network Gate is both:

- Policy Decision Point (PDP): decides allow/deny and route selection.
- Policy Enforcement Point (PEP): enforces budgets, timeouts, redactions,
  and route choice.

Complete mediation is required: no external egress may bypass the Gate.
