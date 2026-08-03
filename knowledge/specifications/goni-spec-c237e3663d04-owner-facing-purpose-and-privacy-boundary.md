---
id: GONI-SPEC-C237E3663D04
title: Owner-facing purpose and privacy boundary
type: specification
status: draft
implementation_state: specified_only
proposition: Receipts exist so the principal and explicitly delegated reviewers can reconstruct meaningful governed transitions.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/receipts.md
  heading: Owner-facing purpose and privacy boundary
  revision: 0b6bf1bf99eef10258d5ea44c7c90bdc24542c70
---

# Owner-facing purpose and privacy boundary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Owner-facing purpose and privacy boundary

Receipts exist so the principal and explicitly delegated reviewers can
reconstruct meaningful governed transitions. They are evidence about what the
system did; they are not truth, moral authority, or accountability merely by
volume.

REC-01 does not create a higher receipt tier merely because generated text is
controversial, heterodox, offensive, or politically sensitive. An ordinary LLM
call may still produce the execution or telemetry evidence required by runtime
policy, but content viewpoint alone MUST NOT create tool authority, external
disclosure, or expanded monitoring.

Receipt collection MUST remain purpose-limited to mediated actions, durable
knowledge changes, policy decisions, and configured operational evidence. It
MUST NOT be interpreted as authority for ambient worker surveillance,
continuous private-activity capture, or collection unrelated to a governed
system transition.

Meaningful knowledge changes such as durable memory commits, ontology merges
or splits, and selection of a controlling operational rule MUST preserve their
source refs, authority basis, policy hash, memory diff refs, and rollback or
undo refs through the applicable existing receipt fields. This adds no receipt
schema fields. Receipt access, retention, and disclosure remain permission- and
policy-scoped by the principal.
