---
id: DELEGATION-THEORY-MAP-01
title: Sovereign human-to-AI delegation theory map
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni integrates reasoning research, agent-system engineering, capability security, mixed-initiative interaction, principal-agent governance, and auditability around one persistent human principal whose delegated authority remains bounded, attenuable, revocable, and reconstructable across replaceable AI substrates.
domains: [research, delegation, system, governance]
aliases:
- sovereign delegation academic map
- human-to-ai delegation theory
relations:
- type: synthesizes
  target: DELEG-MODEL-01
- type: synthesizes
  target: DELEG-AUTHORITY-01
- type: synthesizes
  target: DELEG-AGENCY-01
- type: synthesizes
  target: DELEG-RESPONSIBILITY-01
- type: synthesizes
  target: AUTON-VECTOR-01
- type: synthesizes
  target: CONFUSED-DEPUTY-AI-01
- type: synthesizes
  target: HARNESS-BOUNDARY-01
- type: synthesizes
  target: AGENT-SYSTEMS-MAP-01
- type: synthesizes
  target: AUDIT-RUNTIME-01
- type: synthesizes
  target: DELEGATION-NEIGHBOR-MAP-2026-01
- type: refines
  target: GONI-THESIS-98AE2B9284ED
sources:
- SRC-JENSEN1976-AGENCY-COSTS
- SRC-HOLMSTROM1979-OBSERVABILITY
- SRC-HARDY1988-CONFUSED-DEPUTY
- SRC-PARASURAMAN2000-AUTOMATION-LEVELS
- SRC-HORVITZ1999-MIXED-INITIATIVE
- SRC-TOMASEV2026-INTELLIGENT-DELEGATION
artifacts: []
uncertainty: This is Goni's current synthesis of several established and emerging traditions, not a claim that the combined framing is unique in every component. Novelty should be tested against current literature and implementations.
legacy: []
---

# Sovereign human-to-AI delegation theory map

Goni's core research object is not the language model or the agent loop. It is the **persistent delegation relationship between a human principal and heterogeneous digital delegates**.

The academic progression is:

```text
structured model cognition
        ↓
agentic reasoning and tool use
        ↓
context, memory, and runtime engineering
        ↓
capability security and authority provenance
        ↓
mixed-initiative human automation
        ↓
principal-delegate governance and observability
        ↓
accountability and reconstructable effects
        ↓
sovereign human-to-AI delegation
```

Each lineage answers a different question:

- **reasoning research** asks how models can reach better conclusions;
- **agent systems** ask how models can maintain state and act over time;
- **harness engineering** asks how context, tools, models, and feedback are composed;
- **capability security** asks what authority software possesses and under which provenance;
- **mixed initiative** asks which functions should be automated and when humans should intervene;
- **principal-agent theory** contributes a language for delegation costs, monitoring, observability, and residual loss;
- **auditability research** asks whether consequential trajectories can later be reconstructed and assessed.

Goni combines these around a single invariant:

[
Authority_{effective}subseteq Authority_{validly delegated}subseteq Authority_{principal}
]

The principal may delegate wide operational freedom, but capability, model strength, provider choice, harness behavior, or subdelegation cannot silently create new authority.

## System consequence

The provider-agnostic control path is:

```text
Principal intent
→ Delegation object
→ WorkOrder / DoneContract
→ Context + cognitive harness
→ Proposed action
→ Kernel authorization against current authoritative state
→ Capability-scoped effect
→ Observation
→ Verification
→ Receipt
→ Governed state update
```

Models reason. Harnesses coordinate cognition. The delegation runtime compiles and manages the work relationship. The kernel authorizes effects. Tools execute. Receipts preserve evidence.

## Research claim discipline

Goni should avoid claiming novelty for individual mechanisms that now have close neighbors: agent kernels, access-control overlays, least-privilege tool policies, agent operating systems, memory systems, and audit architectures.

The stronger research question is:

> **How can a human delegate persistent digital agency across heterogeneous probabilistic systems while retaining sovereignty over identity, memory, authority, resources, revocation, and consequences?**

That is the organizing question for future architecture and empirical evaluation.
