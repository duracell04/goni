---
id: GONI-SPEC-97314A240F37
title: 2. Example (YAML)
type: specification
status: draft
implementation_state: specified_only
proposition: 2. Example (YAML)
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
  heading: 2. Example (YAML)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 2. Example (YAML)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Example (YAML)

```yaml
id: goni.agent.local_researcher
version: 0.1.0
description: Index reports and draft weekly summaries.
triggers:
  - type: folder_changed
    path: ~/Documents/Reports
capabilities:
  fs_read: [~/Documents/Reports]
  fs_write: [~/Documents/Summaries]
  network: false
budgets:
  solver_wake_per_hour: 6
  max_exec_time_s: 120
  max_ssd_writes_per_day_mb: 200
policy_profile:
  data_scopes: [documents, notes]
  tags: [local_only]
ui_surfaces:
  - dashboard_tile
  - inbox_sidebar
identity_requirements:
  - user_session
remote_access: false
tools:
  - pdf_text_extract
  - vecdb_upsert
  - report_writer
```
