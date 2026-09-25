---
id: SUPERVISORY-INTERACTION-01
title: Supervisory interaction and voice teaching
type: specification
status: draft
implementation_state: specified_only
proposition: Natural-language and voice interaction may supervise Goni by correcting outputs, refining workflow expectations, and proposing policy or corridor changes, while spoken or textual corrections are compiled into reviewable procedural or authority proposals rather than silently becoming new authority.
domains:
- specs
- interaction
- delegation
aliases:
- voice-supervision
- supervisory-voice
relations:
- type: depends_on
  target: ANTICIPATORY-DELEGATION-01
- type: refines
  target: CDC-01
sources:
- SRC-HORVITZ1999-MIXED-INITIATIVE
artifacts: []
uncertainty: Voice-specific confirmation ergonomics and authentication requirements depend on device, environment, consequence level, and threat model.
legacy: []
---

# Supervisory interaction and voice teaching

Voice and conversational interfaces are supervisory channels over the same Goni
authority model as graphical or API interactions.

The principal may use natural language to:

- correct a draft or interpretation,
- identify a repeated workflow,
- state that a preparation step should happen automatically in comparable cases,
- narrow or broaden a workflow template,
- propose a corridor or approval-policy change,
- revoke or suspend delegated behavior.

The runtime must distinguish procedural learning from authority modification.

A correction such as "next time prepare this automatically" may become a
procedural-memory or WorkflowTemplate proposal. A statement that changes
external-action authority must compile into the appropriate explicit policy,
mandate, corridor, or capability change and follow its normal governance path.

Voice is therefore a low-friction teaching and supervision modality, not an
authority bypass.
