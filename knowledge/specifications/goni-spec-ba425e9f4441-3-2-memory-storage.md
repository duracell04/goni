---
id: GONI-SPEC-BA425E9F4441
title: 3.2 Memory & Storage
type: specification
status: draft
implementation_state: specified_only
proposition: 'The system must: Provide enough **system memory** and/or **unified memory** to: host at least one medium-to-large model (tens of billions of parameters) in compressed form, maintain in-memory indices and caches for personal data, run supporting services without constant swapping.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/10-requirements.md
  heading: 3.2 Memory & Storage
  revision: a37b40b24ee0d0c5351b8fcb8023917007aa3768
---

# 3.2 Memory & Storage

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 Memory & Storage

The system must:

- Provide enough **system memory** and/or **unified memory** to:
  - host at least one medium-to-large model (tens of billions of parameters) in compressed form,
  - maintain in-memory indices and caches for personal data,
  - run supporting services without constant swapping.

- Provide persistent **storage** for:
  - operating system and base software,
  - model files for multiple models,
  - embeddings / indices,
  - user data and configuration.

Storage requirements should assume:

- Multi-terabyte local storage as a baseline.
- At least **one expansion path** (e.g. an extra internal slot) for future capacity increases.
