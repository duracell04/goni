---
id: GONI-SYNTHESIS-75B6F5354041
title: 'Policy: burst protection'
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Status: specified only / roadmap Rules: Clamp background admission during bursts.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/scheduler/policies/burst_protection.md
  heading: 'Policy: burst protection'
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# Policy: burst protection

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# Policy: burst protection

Status: specified only / roadmap

Rules:
- Clamp background admission during bursts.
- Re-open background admission once queue recovers.
