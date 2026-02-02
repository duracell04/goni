# AGENT-02 - Agent Manifest
DOC-ID: AGENT-MANIFEST-01
Status: Specified only / roadmap

The agent manifest is the single source of truth for agent identity, triggers,
capabilities, and budgets. The kernel refuses to instantiate agents whose
requested capabilities exceed policy.

Canonical data contract: `software/50-data/51-schemas-mvp.md` (MANIFEST-02).

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

## 3. Invariants

- **No ambient authority:** capabilities must be explicitly listed.
- **Budget required:** an agent without budgets is invalid.
- **Default-off network:** network access is false unless explicitly granted.
- **UI integration is explicit:** any UI surface must be declared in the manifest.
- **Remote access is explicit:** remote presence requires an explicit grant.
- **Policy intersection:** effective capabilities are the intersection of
  manifest requests and active policy.

## 4. Schema and audit linkage

The canonical record is `AgentManifests` (see
`software/50-data/51-schemas-mvp.md` for the storage schema). The following
audit fields are required:

- `agent_id`
- `policy_hash`
- `state_snapshot_id`
- `provenance`

## 5. Related specs

- [agent-definition.md](./agent-definition.md)
- [tool-capability-api.md](./tool-capability-api.md)

## 6. Upstream
- [Agent definition](./agent-definition.md)
- [Tool capability API](./tool-capability-api.md)
- [Schema MVP](../../software/50-data/51-schemas-mvp.md)

## 7. Downstream
- [Scheduler and interrupts](./scheduler-and-interrupts.md)
- [Orchestrator](../../software/30-components/orchestrator.md)

## 8. Adjacent
- [Receipts](./receipts.md)
- [System map](../00-system-map.md)

## Conformance tests
- TBD: add tests for this spec.




