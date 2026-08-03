---
id: GONI-SYNTHESIS-4AE05DA9A849
title: 2. Kernel properties required
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'The trusted core is modeled as a reference-monitor style mechanism with three required properties: always invoked (complete mediation), tamper resistant, small enough to analyze and validate.'
domains:
- agent
- kernel
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/40-agentic-kernel-foundations.md
  heading: 2. Kernel properties required
  revision: 674844ea4542b314220f725c14edb1c256c1856c
---

# 2. Kernel properties required

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Kernel properties required

The trusted core is modeled as a reference-monitor style mechanism with three
required properties:
- always invoked (complete mediation),
- tamper resistant,
- small enough to analyze and validate.

This aligns with reference-monitor requirements and design principles in
classical OS security work [[anderson1972-reference-monitor]]
[[saltzer1975-protection]].

In Personal AI Ledger (PAL) language, this trusted core is the local Tool
Gateway: the mandatory reference monitor between the model/runtime and external
objects such as files, shell commands, memory stores, networks, browsers,
calendars, and other tools. Goni keeps the canonical authority in the kernel
rather than in the LLM, a third-party agent framework, or a tool host.

The trusted core is also the only authority layer. External agent frameworks,
model gateways, runtimes, vector stores, UIs, voice systems, workflow engines,
and hardware accelerators are replaceable substrates. They may perform work
behind adapters, but they must not own canonical memory, identity, permissions,
policy, approvals, receipts, or rollback state.

Kernel modularity rule:
- interfaces precede implementations,
- substrate state is non-canonical unless promoted through Goni contracts,
- remote routes require policy-approved and receipted egress,
- each external layer must remain swappable,
- promotion toward the core requires evidence and a rollback path.

PAL's three-layer action model is a non-normative way to explain the same
kernel discipline:

- Thinking: reasoning, retrieval, drafts, summaries, classification, and other
  low-risk internal work. These events may be logged or summarized by policy,
  but they do not directly mutate external state.
- Proposal: pre-execution intent made reviewable as a Work Order or proposal.
  It declares the target, reason, expected change, risk, approval need, and
  rollback or compensation plan.
- Commitment: an approved mediated side effect. The kernel applies the tool
  operation, emits the canonical receipt, records hashes and snapshots or diffs
  where applicable, and attaches rollback or compensation references.

This is a 2PC-inspired commit discipline, not a claim of database-style
atomicity across files, networks, people, and external services.
