---
id: GONI-SPEC-147BB23287D1
title: Requirements
type: specification
status: draft
implementation_state: specified_only
proposition: tools execute in isolated processes or containers root filesystem is read-only by default outbound network denied unless via egress gate
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/isolation-and-tool-sandboxes.md
  heading: Requirements
  revision: 9a29f6eb9fee912e41d8e4c7aa0b325aff6cf7b2
---

# Requirements

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Requirements
- tools execute in isolated processes or containers
- root filesystem is read-only by default
- outbound network denied unless via egress gate
