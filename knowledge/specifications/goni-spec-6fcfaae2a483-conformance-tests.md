---
id: GONI-SPEC-6FCFAAE2A483
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: visual actions require a Work Order and Done Contract private/person/evidence assets require stricter gates than public references generated or edited outputs require source hashes, workflow hashes, output hashes, and rollback refs visual receipts must omit raw private content by default model bundles may only run allowed visual task classes
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/visual-intelligence-plane.md
  heading: Conformance tests
  revision: 024fa5a7ee1a38fe8275f518728449c95be3d76c
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests

- visual actions require a Work Order and Done Contract
- private/person/evidence assets require stricter gates than public references
- generated or edited outputs require source hashes, workflow hashes, output
  hashes, and rollback refs
- visual receipts must omit raw private content by default
- model bundles may only run allowed visual task classes
- third-party workflow logs cannot replace Goni receipts
- legal/evidence visual tasks must run in audit-grade mode
- analysis, generation, editing/transformation, evidence annotation, and
  screenshot/design audit must preserve distinct permission and receipt
  requirements
- visual memory writes must preserve rights status, permission scope,
  provenance, and receipt refs
- verification failures must block, retry, request approval, or downgrade the
  deliverable rather than being silently ignored
- visual analysis must not imply memory write, remote extraction, or synthetic
  input authority
