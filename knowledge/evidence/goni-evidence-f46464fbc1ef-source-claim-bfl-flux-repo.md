---
id: GONI-EVIDENCE-F46464FBC1EF
title: 'Source claim: bfl-flux-repo'
type: evidence
status: draft
implementation_state: not_applicable
proposition: The FLUX official inference repository documents available FLUX.1 model variants, model links, and license differences, including Apache-licensed and non-commercial variants.
domains:
- research
aliases: []
relations:
- type: supports
  target: MODEL-REG-01
- type: supports
  target: VIS-01
sources:
- SRC-BFL-FLUX-REPO
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[bfl-flux-repo]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: bfl-flux-repo

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[bfl-flux-repo]]
Claim: The FLUX official inference repository documents available FLUX.1 model
variants, model links, and license differences, including Apache-licensed and
non-commercial variants.
Relevance:
- Grounds visual model bundle license-state tracking for image generation
  substrate.
- Supports VIS-01's requirement that visual model families enter Goni through
  governed bundle manifests rather than informal model names.
Used in:
- `blueprint/30-specs/visual-intelligence-plane.md` (Execution substrate)
- `blueprint/30-specs/model-registry.md` (Visual bundle metadata)
Source:
- https://github.com/black-forest-labs/flux
