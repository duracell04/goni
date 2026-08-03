---
id: GONI-SPEC-BC9AB93EE563
title: 0.1 Expression/effects boundary
type: specification
status: draft
implementation_state: specified_only
proposition: Private local computation and expression are not tool syscalls.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/tool-capability-api.md
  heading: 0.1 Expression/effects boundary
  revision: 8f80e89d99741299556b1ebbc7966bdd71ed4c18
---

# 0.1 Expression/effects boundary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 0.1 Expression/effects boundary

Private local computation and expression are not tool syscalls. Generating,
analyzing, criticizing, summarizing, or drafting text inside ephemeral model or
Context Plane state requires no TOOL-01 capability. The content's viewpoint,
offensiveness, political sensitivity, or heterodoxy does not convert it into an
effectful action.

A proposal also carries no execution authority. Capability mediation begins
when the system attempts to commit durable state or impose an effect outside
the ephemeral reasoning context. Examples include durable memory or ontology
writes, filesystem mutation, sending or publishing content, network calls,
payments, deletion, synthetic input, and device or robot actuation.

Draft location matters. Producing a local string that could become an email is
expression; creating a draft in an external mail account is an external state
change and therefore a tool call. No prompt, model output, source text, or
content classification can grant the capability needed for that transition.
