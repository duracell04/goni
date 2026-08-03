---
id: GONI-SYNTHESIS-20D078B8208A
title: Security constraints
type: synthesis
status: draft
implementation_state: specified_only
proposition: No secrets or credentials in repo.
domains:
- agent
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/meta/agents.root.template.md
  heading: Security constraints
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Security constraints

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Security constraints
- No secrets or credentials in repo.
- No outbound telemetry by default; any outbound access must be explicit and documented.
