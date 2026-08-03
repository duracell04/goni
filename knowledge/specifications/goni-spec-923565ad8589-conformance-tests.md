---
id: GONI-SPEC-923565AD8589
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: direct socket use from tool sandbox must fail egress via gate succeeds with valid capability unavailable required isolation fails closed irreversible high-risk actions require explicit approval evidence and dual receipts screen capture/extraction sandboxes cannot write memory or synthesize input without separate grants desktop/browser actuation requires an actuation grant and fails closed if the
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/isolation-and-tool-sandboxes.md
  heading: Conformance tests
  revision: 9a29f6eb9fee912e41d8e4c7aa0b325aff6cf7b2
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests
- direct socket use from tool sandbox must fail
- egress via gate succeeds with valid capability
- unavailable required isolation fails closed
- irreversible high-risk actions require explicit approval evidence and dual
  receipts
- screen capture/extraction sandboxes cannot write memory or synthesize input
  without separate grants
- desktop/browser actuation requires an actuation grant and fails closed if the
  required sandbox profile is unavailable
