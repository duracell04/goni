---
id: GONI-SYNTHESIS-1025792B19DF
title: Enforcement
type: synthesis
status: draft
implementation_state: specified_only
proposition: CI blocks any commit that adds a concrete model size, exact dBA number, or specific CPU/GPU part name to README.md, blueprint/docs/goni-story.md, or blueprint/docs/goni-whitepaper.md (simple grep check in /.github/workflows/ci.yml).
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/goni-agility-rules.md
  heading: Enforcement
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Enforcement

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Enforcement

1. CI blocks any commit that adds a concrete model size, exact dBA number, or specific CPU/GPU part name to `README.md`, `blueprint/docs/goni-story.md`, or `blueprint/docs/goni-whitepaper.md` (simple grep check in `/.github/workflows/ci.yml`).
2. Any PR that wants to tighten a constraint (e.g. “change target volume from ≤ 10 L to ≤ 8 L”) must:
   - update `blueprint/hardware/10-requirements.md` or `blueprint/software/90-decisions.md` only,
   - include a photo / measurement / benchmark proving the new number is real today,
   - keep a one-line changelog with date and a **valid until** line.
