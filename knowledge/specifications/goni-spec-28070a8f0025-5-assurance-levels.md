---
id: GONI-SPEC-28070A8F0025
title: 5. Assurance levels
type: specification
status: draft
implementation_state: specified_only
proposition: 'Goni uses graded assurance, not a trusted/untrusted binary: | Level | Evidence | Maximum default use | | L0 | Hash only | Sandbox testing | | L1 | Hash + manifest + license state | Public or low-risk tasks | | L2 | Local eval receipt | Personal low-sensitivity memory | | L3 | Signed third-party or community eval | Broader tool use |'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: 5. Assurance levels
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# 5. Assurance levels

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Assurance levels

Goni uses graded assurance, not a trusted/untrusted binary:

| Level | Evidence | Maximum default use |
|-------|----------|---------------------|
| L0 | Hash only | Sandbox testing |
| L1 | Hash + manifest + license state | Public or low-risk tasks |
| L2 | Local eval receipt | Personal low-sensitivity memory |
| L3 | Signed third-party or community eval | Broader tool use |
| L4 | Reproducible provenance + ML-BOM + audit trail | Sensitive memory or enterprise use |

Assurance levels are permission ceilings, not guarantees. Policy may further
restrict a bundle below its assurance level.
