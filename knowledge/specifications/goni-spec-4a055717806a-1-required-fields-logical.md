---
id: GONI-SPEC-4A055717806A
title: 1. Required fields (logical)
type: specification
status: draft
implementation_state: specified_only
proposition: 'id: stable agent identifier (reverse-DNS).'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agent-manifest.md
  heading: 1. Required fields (logical)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 1. Required fields (logical)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Required fields (logical)

- `id`: stable agent identifier (reverse-DNS).
- `version`: semantic version.
- `description`: short purpose statement.
- `triggers`: event and schedule conditions.
- `capabilities`: scoped tool permissions.
- `budgets`: solver calls, runtime, disk/network ceilings.
- `policy_profile`: data scopes and privacy tags.
- `ui_surfaces`: declared UI integration points.
- `identity_requirements`: required identity context.
- `remote_access`: whether remote presence can be requested.
- `tools`: optional tool preferences (non-binding).
