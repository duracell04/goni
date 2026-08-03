---
id: GONI-SPEC-6686192E5EF4
title: 7. Autonomy packs and templates
type: specification
status: draft
implementation_state: specified_only
proposition: Goni SHOULD support profile-based policy packs and SOP templates so users can start with safe defaults and adapt over time.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 7. Autonomy packs and templates
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 7. Autonomy packs and templates

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Autonomy packs and templates

Goni SHOULD support profile-based policy packs and SOP templates so users can
start with safe defaults and adapt over time.

Each imported pack MUST declare:

- covered task classes,
- default corridors and thresholds,
- declared no-go actions,
- provenance (pack version, author, policy hash).

Packs MAY also declare delegation-policy defaults for clarification thresholds,
assumption visibility, and irreversible-action rules.
