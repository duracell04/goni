---
id: GONI-SPEC-FE4FB11BB885
title: 4. Boundary objects
type: specification
status: draft
implementation_state: specified_only
proposition: Every mediated transition SHOULD preserve compact refs for the following objects.
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
  heading: 4. Boundary objects
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 4. Boundary objects

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Boundary objects

Every mediated transition SHOULD preserve compact refs for the following
objects. Raw private content is not stored in Control-plane fields by default.

- `observation_scope`: permitted app, window, tab, screen, monitor, event
  stream, accessibility tree, frame rate class, time window, and revocation
  rule.
- `extraction_profile`: permitted extraction modes such as OCR, accessibility
  parse, layout parse, summarization, embedding, remote model submission,
  redaction profile, and output shape.
- `memory_grant`: permitted memory class, source refs, retention/expiry,
  indexing rule, sync posture, review status, and tombstone behavior.
- `actuation_grant`: permitted tool IDs, synthetic input classes, filesystem or
  network scopes, irreversible-action rules, idempotency rule, and allowed side
  effects.
- `sandbox_profile`: required process, container, browser, microVM, or OS
  isolation boundary for the current action class.
- `approval_requirement`: no approval, queued review, soft gate, hard gate,
  two-phase commit, or explicit human confirmation.
- `receipt_requirement`: receipt tier and required basis fields for the
  boundary transition.
- `rollback_or_repair_ref`: snapshot, undo strategy, compensation path, repair
  workflow, or explicit statement that no rollback exists.
