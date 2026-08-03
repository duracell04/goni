---
id: GONI-SYNTHESIS-0E232A349974
title: 'Policy: interactive-first'
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Status: specified only / roadmap Rules: Always admit interactive requests.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/scheduler/policies/interactive_first.md
  heading: 'Policy: interactive-first'
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# Policy: interactive-first

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Policy: interactive-first

Status: specified only / roadmap

Rules:
- Always admit interactive requests.
- Throttle background when interactive queue is non-empty.
