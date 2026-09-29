---
id: GONI-IMAP-216164F1995B
title: 0.0a Harness Plane (governance overlay)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The model is not the agent; the governed harness must provide both sufficient authority-bounded actuation and sufficient observability to verify the delegated outcome.
domains:
- software
aliases: []
relations:
- type: depends_on
  target: GONI-SPEC-F37FC6D98E05
- type: depends_on
  target: GONI-SPEC-59E3DFC23CC8
sources: []
artifacts: []
uncertainty: The harness plane remains a conceptual governance overlay. Capability discovery, harness routing, and verification-path selection are not yet implemented or validated.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 0.0a Harness Plane (governance overlay)
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 0.0a Harness Plane (governance overlay)

> Status boundary: this is a specified-only implementation map. Enforcement
> language describes intended conformance behavior rather than observed
> implementation.

## 0.0a Harness Plane (governance overlay)

The model is not the agent. The agent is the model plus harness: the governed
operating substrate that selects context, retrieves memory, exposes tools,
routes models, applies approval corridors, writes receipts, evaluates outcomes,
and rolls back failed changes.

We call this conceptual overlay the **Goni Harness Plane**. It does not change
the formal node tuple \(N = (\mathcal{A}, \mathcal{X}, \mathcal{K},
\mathcal{E})\) in this version. Instead, it names a cross-plane governance
contract:

- Context Plane: what evidence and memory enter the model context.
- Control Plane: when Goni asks, assumes, escalates, schedules, or interrupts.
- Execution Substrate: which models, tools, and sandboxes are available.
- Policy and receipts: what authority is granted, what must be recorded, and
  what rollback path exists.

## Actuation and observability fit

Harness suitability has two independent dimensions:

1. **Actuation fit:** can this environment perform the authorized state change?
2. **Observation fit:** can this environment observe and verify the postconditions
   required by the DoneContract?

Tool availability alone is therefore insufficient. A deployment connector may
be able to create a deployment while lacking an independent HTTP or browser
path capable of establishing that the user-facing application works. A mail
tool may be able to submit a message while a separate readback path is needed
to establish the provider-visible sent state.

The Control Plane SHOULD select the smallest harness that provides:

- the authority-bounded actuation capabilities required by the WorkOrder;
- the observation and verification capabilities required by its DoneContract;
- the isolation, rollback, identity, and network properties required by policy.

A harness MUST NOT certify a consequential task as complete when its own
capability boundary prevents the required verification. It may report the
narrower state it actually established.

This yields the compact rule:

> **Minimum sufficient authority plus minimum sufficient observability.**

Harness components are versioned artefacts, not hidden glue. Prompts, context
assembly templates, retrieval policies, routing thresholds, tool manifests,
approval corridors, receipt formats, verification adapters, and eval packs must
be inspectable and reversible. A harness change is promoted only when it
declares an expected effect, measures that effect against receipt-backed
evidence, and retains or rolls back according to the evaluation result.

This keeps the formal architecture stable while making agent competence and
completion confidence observable systems properties rather than unexplained
model behavior.

---
