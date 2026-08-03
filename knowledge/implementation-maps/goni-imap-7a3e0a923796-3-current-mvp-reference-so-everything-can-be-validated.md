---
id: GONI-IMAP-7A3E0A923796
title: 3. Current MVP reference (so everything can be validated)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'See 90-decisions.md for the canonical decisions, but in one line: **MVP reference compute module:** APU-centric node based on a **Ryzen AI Max+ 395 class** board with **128 GB unified LPDDR5X** (Framework Desktop mainboard is the primary reference; HP Z2 Mini G1a is the off-the-shelf fallback).'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/00-overview.md
  heading: 3. Current MVP reference (so everything can be validated)
  revision: a7f653c2ecb06e74e76c340525db7b4d6a7c10ec
---

# 3. Current MVP reference (so everything can be validated)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Current MVP reference (so everything can be validated)

See [`90-decisions.md`](/blueprint/hardware/90-decisions.md) for the canonical decisions, but in one line:

- **MVP reference compute module:** APU-centric node based on a **Ryzen AI Max+ 395 class** board with **128 GB unified LPDDR5X** (Framework Desktop mainboard is the primary reference; HP Z2 Mini G1a is the off-the-shelf fallback).  
- **Supported minimum:** 64 GB unified-memory devices may be used for early development and testing, but they are not performance-representative for the product story and must be treated as a degraded mode (see `blueprint/software/10-requirements.md`).
- **MVP enclosure envelope:** ~7 L, quiet, front status bar, internal SFX PSU, 2?? NVMe.  
- **MVP networking:** 5 GbE preferred (2.5 GbE acceptable only as fallback for early dev boxes).

---
