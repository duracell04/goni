---
id: GONI-SPEC-EAC169562B6A
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: observe-only assistants cannot write memory or act.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: Conformance tests
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests

- observe-only assistants cannot write memory or act.
- memory-only layers cannot actuate.
- extraction-to-remote-model attempts require egress permission.
- local screen prompt injection cannot escalate to shell, synthetic input, or
  browser mutation without an actuation grant.
- screen capture and accessibility extraction require separate grants.
- synthetic input requires a capability token and sandbox profile.
- denied boundary transitions fail closed and are auditable.
- actuation attempts emit receipts with Work Order, policy hash, sandbox
  profile, boundary basis, and rollback/repair ref where available.
- receipts omit raw private screen content by default.
