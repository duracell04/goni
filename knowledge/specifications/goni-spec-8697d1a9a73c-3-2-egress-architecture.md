---
id: GONI-SPEC-8697D1A9A73C
title: 3.2 Egress architecture
type: specification
status: draft
implementation_state: specified_only
proposition: Only mediation gateway and egress gateway are allowed outbound routes.
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-ENF-01-non-bypassable-mediation.md
  heading: 3.2 Egress architecture
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 3.2 Egress architecture

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Egress architecture

Only mediation gateway and egress gateway are allowed outbound routes.
Tool runners have:
- no direct outbound socket permission,
- explicit proxy path for permitted calls,
- trace/capability metadata attached per request.
