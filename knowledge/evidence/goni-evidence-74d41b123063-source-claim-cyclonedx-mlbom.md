---
id: GONI-EVIDENCE-74D41B123063
title: 'Source claim: cyclonedx-mlbom'
type: evidence
status: draft
implementation_state: not_applicable
proposition: CycloneDX ML-BOM represents models, datasets, dependencies, dataset provenance, training methodologies, and AI framework configuration for transparency and risk assessment.
domains:
- research
aliases: []
relations:
- type: supports
  target: MODEL-REG-01
sources:
- SRC-CYCLONEDX-MLBOM
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[cyclonedx-mlbom]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: cyclonedx-mlbom

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[cyclonedx-mlbom]]
Claim: CycloneDX ML-BOM represents models, datasets, dependencies, dataset
provenance, training methodologies, and AI framework configuration for
transparency and risk assessment.
Relevance:
- Supports treating model provenance as machine-readable supply-chain metadata.
- Provides a basis for making ML-BOM data an input to local policy checks.
Used in:
- `blueprint/30-specs/model-registry.md` (Evaluation limits)
Source:
- https://cyclonedx.org/capabilities/mlbom/
