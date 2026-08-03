---
id: GONI-IMAP-216164F1995B
title: 0.0a Harness Plane (governance overlay)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The model is not the agent.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 0.0a Harness Plane (governance overlay)
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 0.0a Harness Plane (governance overlay)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

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

Harness components are versioned artefacts, not hidden glue. Prompts, context
assembly templates, retrieval policies, routing thresholds, tool manifests,
approval corridors, receipt formats, and eval packs must be inspectable and
reversible. A harness change is promoted only when it declares an expected
effect, measures that effect against receipt-backed evidence, and retains or
rolls back according to the evaluation result.

This keeps the formal architecture stable while making agent competence an
observable systems property rather than an unexplained model behavior.

---
