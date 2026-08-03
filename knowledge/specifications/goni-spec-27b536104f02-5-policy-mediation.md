---
id: GONI-SPEC-27B536104F02
title: 5. Policy mediation
type: specification
status: draft
implementation_state: specified_only
proposition: The Desktop Agent Firewall is a Policy Decision Point and Policy Enforcement Point for boundary transitions.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 5. Policy mediation
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 5. Policy mediation

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Policy mediation

The Desktop Agent Firewall is a Policy Decision Point and Policy Enforcement
Point for boundary transitions. Policy MUST decide at least:

- default observation scope,
- allowed extraction modes,
- memory classes allowed per task class,
- actuation classes allowed per autonomy corridor,
- remote extraction and egress rules,
- sandbox profile by action class,
- approval requirements,
- receipt tier and retention posture,
- default denial for observation-to-actuation escalation.

If policy is missing or cannot be loaded, the boundary transition is denied.
