---
id: GONI-SYNTHESIS-E37F99C2D44E
title: OpenClaw (agent gateway + channel integrations)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'What it is: A self-hosted agent gateway that integrates chat channels and tools.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: OpenClaw (agent gateway + channel integrations)
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# OpenClaw (agent gateway + channel integrations)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### OpenClaw (agent gateway + channel integrations)

What it is:

- A self-hosted agent gateway that integrates chat channels and tools.
- A practical "operator front door" with messaging and browser automation.

Where it maps in Goni terms:

- Closest to "gateway + tool seats + channel adapters."
- Not a kernel-governed plane model (no receipts, confinement, or capability
  syscalls as first-class primitives in the public framing).
- Useful for interaction and routing ideas, but not a sovereign base: Goni must
  still own authority, receipt semantics, corridor policy, and durable memory.

Refined Goni takeaway:

- "Goni Claw" is the pattern where Goni keeps the sovereign kernel and an
  OpenClaw-like layer supplies the front-door UX.
- That means:
  - chat/channel routing and action surfaces may look gateway-like,
  - but capability checks, corridor outcomes, receipts, and memory provenance
    still terminate in the Goni kernel.
- In short: OpenClaw as surface inspiration, Goni as control plane.

Links:

- https://openclaw.ai/
- https://docs.openclaw.ai/
