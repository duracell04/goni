---
id: DELEGATION-NEIGHBOR-MAP-2026-01
title: 2026 delegation and agent-governance neighbor map
type: synthesis
status: draft
implementation_state: specified_only
proposition: Goni's closest research neighbors increasingly cover agent operating systems, mandatory trust kernels, least-privilege tool authorization, authenticated delegation, and minimal agent harnesses; Goni should therefore position its contribution around persistent sovereign human-to-AI delegation rather than generic agent-OS governance.
domains: [research, system, delegation, market]
aliases:
- delegation neighbor map
- 2026 agent governance landscape
relations:
- type: synthesizes
  target: EVID-AGENT-OPERATING-SYSTEMS-2026-01
- type: synthesizes
  target: EVID-AGENTKERNEL-2026-01
- type: synthesizes
  target: EVID-PROGENT-01
- type: synthesizes
  target: EVID-MINISCOPE-01
- type: synthesizes
  target: EVID-PI-MINIMAL-HARNESS-01
- type: refines
  target: GONI-SYNTHESIS-8A2FFAA7A328
sources:
- SRC-SHARMA2026-AGENT-OPERATING-SYSTEMS
- SRC-ZOU2026-AGENTKERNEL
- SRC-SHI2025-PROGENT
- SRC-ZHU2025-MINISCOPE
- SRC-PI2026-MINIMAL-AGENT-HARNESS
artifacts: []
uncertainty: This map reflects reviewed sources available by 2026-09-26 and should be updated as the field changes. It does not rank systems or assert production maturity.
legacy: []
---

# 2026 delegation and agent-governance neighbor map

The relevant landscape is no longer separated cleanly into "agents" and "operating systems."

| Neighbor class | Representative source | Overlap with Goni | Provider-agnostic implication |
| --- | --- | --- | --- |
| Agent operating system | Sharma and Shah 2026 | scheduling, context, memory, capability registry, policy, audit | OS framing alone is not sufficient novelty |
| Trust-native agent kernel | AgentKernel | identity, perception, memory governance, execution control | kernelized governance is an adjacent research direction |
| Least-privilege tool control | Progent | deterministic privilege enforcement around tool calls | authority can live outside model reasoning |
| Least-privilege permission hierarchy | MiniScope | bounded tool permissions | tool visibility and tool authority should remain distinct |
| Minimal agent harness | Pi | models, tools, sessions, skills, extension surface | harnesses can remain replaceable beneath sovereign control |

Goni's strongest differentiating research target is therefore:

> **persistent sovereign human-to-AI delegation across heterogeneous cognitive and execution substrates.**

The emphasis is the durable principal relationship: goals, identity, memory governance, authority lineage, revocation, accountability, and continuity should survive changes in model, provider, harness, or tool implementation.
