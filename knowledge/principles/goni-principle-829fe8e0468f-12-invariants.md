---
id: GONI-PRINCIPLE-829FE8E0468F
title: 12. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: Visual actions require a Work Order and Done Contract.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: 12. Invariants
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# 12. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 12. Invariants

- Visual actions require a Work Order and Done Contract.
- Private, person-identifying, brand-sensitive, and legal/evidence assets
  require stricter gates than public reference assets.
- Generated or edited outputs require source hashes, workflow hashes, output
  hashes, and rollback refs when a prior version exists.
- Visual receipts must omit raw private content by default.
- Model bundles may only run visual task classes allowed by MODEL-REG-01.
- Workflow backends and third-party logs cannot replace canonical Goni
  receipts.
- Visual memory stores governed metadata and refs; raw binaries remain
  content-addressed artifacts outside Control-plane records.
- Mixed visual tasks inherit the strictest permission posture, receipt fields,
  and verification requirements among their task profiles.
- Visual observation, extraction, memory, remote submission, and actuation
  remain separate governed powers under BOUND-01.
