---
id: GONI-IMAP-9763D2C3DDB0
title: Data Plane
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '[ ] Hot-path transforms are enumerated and documented as elements of \(\mathcal{A}^{\text{hot}}\).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-conformance.md
  heading: Data Plane
  revision: 3f25365c21d9b87a7a295e5ec9e9221e34e8958e
---

# Data Plane

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Data Plane

- [ ] Hot-path transforms are enumerated and documented as elements of \(\mathcal{A}^{\text{hot}}\).  
- [ ] Zero-copy invariant A1 is empirically tested on random batches.  
- [ ] Affine-use invariant A2 holds for all tested compositions on hot paths.
