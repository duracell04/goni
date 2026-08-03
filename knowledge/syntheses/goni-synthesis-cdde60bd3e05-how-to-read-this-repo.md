---
id: GONI-SYNTHESIS-CDDE60BD3E05
title: How to read this repo
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Product/story track (stakeholders, early adopters)**: start with blueprint/docs/goni-story.md, then blueprint/docs/goni-whitepaper.md for the deep architecture narrative, blueprint/20-system/60-cognitive-exocortex-model.md for the first/second/third-brain crosswalk, blueprint/20-system/65-local-sovereign-knowledge-runtime.md for the local-expression and governed-effects posture, blueprint/10-product/05-sovereign-delegation-os-thesis.md for the academic delegation thesis, and blueprint/docs/gon'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: How to read this repo
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# How to read this repo

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## How to read this repo

- **Product/story track (stakeholders, early adopters)**: start with [blueprint/docs/goni-story.md](/blueprint/docs/goni-story.md), then [blueprint/docs/goni-whitepaper.md](/blueprint/docs/goni-whitepaper.md) for the deep architecture narrative, [blueprint/20-system/60-cognitive-exocortex-model.md](/blueprint/20-system/60-cognitive-exocortex-model.md) for the first/second/third-brain crosswalk, [blueprint/20-system/65-local-sovereign-knowledge-runtime.md](/blueprint/20-system/65-local-sovereign-knowledge-runtime.md) for the local-expression and governed-effects posture, [blueprint/10-product/05-sovereign-delegation-os-thesis.md](/blueprint/10-product/05-sovereign-delegation-os-thesis.md) for the academic delegation thesis, and [blueprint/docs/goni-swot.md](/blueprint/docs/goni-swot.md) for positioning.
- **Hardware track (hardware builders)**: [blueprint/hardware/00-overview.md](/blueprint/hardware/00-overview.md) -> [blueprint/hardware/10-requirements.md](/blueprint/hardware/10-requirements.md) -> [blueprint/hardware/20-architecture-options.md](/blueprint/hardware/20-architecture-options.md) -> [blueprint/hardware/25-hardware-layers-and-supplier-map.md](/blueprint/hardware/25-hardware-layers-and-supplier-map.md), with accepted choices in [blueprint/hardware/90-decisions.md](/blueprint/hardware/90-decisions.md).
- **Software track (software builders)**: [blueprint/software/00-overview.md](/blueprint/software/00-overview.md) -> [blueprint/software/10-requirements.md](/blueprint/software/10-requirements.md) -> [blueprint/software/20-architecture.md](/blueprint/software/20-architecture.md) -> data spine in [blueprint/software/50-data/00-index.md](/blueprint/software/50-data/00-index.md) (and [blueprint/software/50-data/53-schema-dsl-and-macros.md](/blueprint/software/50-data/53-schema-dsl-and-macros.md) for the Arrow DSL) -> accepted choices in [blueprint/software/90-decisions.md](/blueprint/software/90-decisions.md).
- **Data spine <-> kernel**: the planes and TXT axiom are defined in [blueprint/software/50-data/10-axioms-and-planes.md](/blueprint/software/50-data/10-axioms-and-planes.md); enforcement is specified only (see goni-prototype-lab:goni-lab/STATUS.md).
- **Runs and deployments (I just want to run something)**: see goni-prototype-lab:deploy/docker-compose.yml and goni-prototype-lab:deploy/k8s/ for current status.

---
