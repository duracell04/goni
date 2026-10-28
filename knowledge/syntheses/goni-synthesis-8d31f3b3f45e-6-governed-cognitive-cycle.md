---
id: GONI-SYNTHESIS-8D31F3B3F45E
title: 6. Governed cognitive cycle
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Goni expresses the cognitive cycle through existing contracts: **Observe** inputs without granting them authority.'
domains:
- system
aliases: []
relations:
- type: synthesizes
  target: DELEG-INT-01
- type: synthesizes
  target: GONI-SPEC-59E3DFC23CC8
- type: synthesizes
  target: DELEG-01
- type: synthesizes
  target: TOOL-01
- type: synthesizes
  target: REC-01
- type: synthesizes
  target: SKILL-REG-01
sources: []
artifacts: []
uncertainty: The three-stage mapping is explanatory. The cited delegation, tool, skill, and receipt mechanisms remain specified-only rather than validated runtime behavior.
legacy:
- path: blueprint/20-system/60-cognitive-exocortex-model.md
  heading: 6. Governed cognitive cycle
  revision: cdf162b26a4fe7d78e6daa6039696e89ee0ef17f
---

# 6. Governed cognitive cycle

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Governed cognitive cycle

Goni expresses the cognitive cycle through existing contracts:

1. **Observe** inputs without granting them authority.
2. **Create or bind a Work Order** that states goal, constraints, risk, expected
   result, policy, and budget.
3. **Retrieve** policy-permitted evidence and memory through bounded dense,
   sparse, graph, and metadata paths.
4. **Verify** selected context against source boundaries, parser confidence,
   validity, contradictions, permissions, and the Done Contract.
5. **Reason and integrate** inside a temporary Context Plane projection. Model
   output remains untrusted until checked by the harness.
6. **Propose or commit** through capability and approval gates. Every mediated
   effect produces the required receipt and rollback or compensation reference.
7. **Consolidate** only approved memory candidates through a separate memory
   grant; discard ordinary working context.
8. **Audit** outcomes, stale claims, contradictions, retrieval quality, policy
   decisions, and receipt completeness.

This cycle is bounded by scheduler, token, time, tool, privacy, and risk budgets.
Stopping, escalation, correction, and human veto are normal outcomes rather
than failures of autonomy.

### Three-stage delegation view (explanatory)

The three-step `Interview -> Playbook / Toolbox -> Proof List` model is a
pedagogical compression of this eight-step cognitive cycle, not a replacement
architecture or a new runtime layer.

| Stage | Goni mapping | Existing contractual home |
| --- | --- | --- |
| Specify | Reconstruct intent from context, ask a decisive question only when needed, and bind a Work Order and stable Done Contract. Reuse versioned SOPs or selected skill fragments as procedural guidance. | `DELEG-INT-01`, `DELEG-01`, `SKILL-REG-01` |
| Execute | Propose tool operations, then apply kernel-owned policy, capability, and transaction gates before external effects. Skill or tool availability does not grant authority. | `TOOL-01`, `SPEC-POL-01`, `SPEC-TXN-01` |
| Verify | Distinguish execution receipts from a task-level assessment of observed results against the Done Contract; preserve evidence and uncertainty. | `REC-01`, `GONI-SPEC-59E3DFC23CC8` |

Feedback may produce a bounded retry, an escalation, or a proposal to update
procedural memory or a reusable SOP. Any durable change or expanded authority
still requires its existing memory-write or kernel-authorization boundary.
