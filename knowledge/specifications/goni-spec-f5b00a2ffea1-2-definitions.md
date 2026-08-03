---
id: GONI-SPEC-F5B00A2FFEA1
title: 2. Definitions
type: specification
status: draft
implementation_state: specified_only
proposition: '**Store**: an index that lists agent packages and metadata.'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agents/agent-store.md
  heading: 2. Definitions
  revision: 17d060e8ac309d5a25765a07f3a00da85d0739e1
---

# 2. Definitions

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Definitions

- **Store**: an index that lists agent packages and metadata.
- **Package**: a signed artifact containing code and a manifest.
- **Publisher**: the entity that signs packages.
- **Trust tier**: a store or publisher classification that controls policy.
