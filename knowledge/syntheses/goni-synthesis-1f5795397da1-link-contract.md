---
id: GONI-SYNTHESIS-1F5795397DA1
title: Link contract
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Every hub or subsystem packet should include: Upstream (what this depends on) Downstream (what depends on this) Adjacent (close neighbors) Avoid plain-text file references when a relative link can be used.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/02-taxonomy-and-ids.md
  heading: Link contract
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# Link contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Link contract
Every hub or subsystem packet should include:
- Upstream (what this depends on)
- Downstream (what depends on this)
- Adjacent (close neighbors)

Avoid plain-text file references when a relative link can be used.
