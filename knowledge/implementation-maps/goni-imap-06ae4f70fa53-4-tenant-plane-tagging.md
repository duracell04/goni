---
id: GONI-IMAP-06AE4F70FA53
title: 4. Tenant & Plane Tagging
type: implementation-map
status: draft
implementation_state: specified_only
proposition: tenant_id remains fixed for single-node deployments; multi-tenant variants must route every write through this field.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/20-spine-and-ids.md
  heading: 4. Tenant & Plane Tagging
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 4. Tenant & Plane Tagging

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Tenant & Plane Tagging

- `tenant_id` remains fixed for single-node deployments; multi-tenant variants must route every write through this field.
- `plane` is enforced by the schema DSL; mismatched plane/table pairs fail compilation.
