---
id: GONI-SPEC-48F01C17D07C
title: 3.7 SOP lifecycle and autonomy packs
type: specification
status: draft
implementation_state: specified_only
proposition: 'The system must treat repeatable workflows as SOPs with lifecycle states: shadow -> approved -> autopilot -> revoked.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 3.7 SOP lifecycle and autonomy packs
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.7 SOP lifecycle and autonomy packs

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.7 SOP lifecycle and autonomy packs

- The system must treat repeatable workflows as SOPs with lifecycle states:
  `shadow` -> `approved` -> `autopilot` -> `revoked`.
- Promotion to autopilot must require a configurable success window and no
  unresolved safety findings.
- The product should provide profile-based starter packs (for example
  student/freelancer/admin-heavy roles) with:
  - default task classes,
  - autonomy corridors,
  - baseline SOP templates.
- Users should be able to import, export, and version SOP packs without editing
  low-level policy files directly.
