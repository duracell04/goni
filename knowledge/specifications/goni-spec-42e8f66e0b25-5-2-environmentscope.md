---
id: GONI-SPEC-42E8F66E0B25
title: 5.2 EnvironmentScope
type: specification
status: draft
implementation_state: specified_only
proposition: EnvironmentScope defines where, when, and around whom a robot may observe, extract, remember, or act.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 5.2 EnvironmentScope
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 5.2 EnvironmentScope

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.2 EnvironmentScope

`EnvironmentScope` defines where, when, and around whom a robot may observe,
extract, remember, or act.

Minimum logical fields:

```yaml
environment_scope:
  environment_ref:
  scope_type: "home | office | warehouse | factory | retail | care_site | outdoor | mixed"
  allowed_zones:
  denied_zones:
  sensitive_zones:
  people_presence_rules:
  pet_or_child_rules:
  private_area_rules:
  time_windows:
  allowed_surface_refs:
  prohibited_object_classes:
  map_ref:
  supervision_posture:
  emergency_stop_ref:
  receipt_ref:
```

Examples of denied or sensitive zones include bedrooms, bathrooms, medical
areas, locked storage, nurseries, financial document areas, restricted
workcells, private offices, hazardous machinery zones, and doors or locks when
policy does not authorize access.
