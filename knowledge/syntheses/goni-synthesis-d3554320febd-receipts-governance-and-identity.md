---
id: GONI-SYNTHESIS-D3554320FEBD
title: Receipts, governance, and identity
type: synthesis
status: draft
implementation_state: specified_only
proposition: '| Project / standard | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence | | AEOESS / Agent Passport System | https://datatracker.ietf.org/doc/draft-pidlisnyi-aps/ | Agent identity, scoped delegation, revocation | Study agent passports, signed authorization, and revocation chains for sovereign agent identity.'
domains:
- research
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/references/gonios-research-neighbor-map.md
  heading: Receipts, governance, and identity
  revision: 08e1061f9ab1e1a95e22a924fdc9970e0585851b
---

# Receipts, governance, and identity

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Receipts, governance, and identity

| Project / standard | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| AEOESS / Agent Passport System | https://datatracker.ietf.org/doc/draft-pidlisnyi-aps/ | Agent identity, scoped delegation, revocation | Study agent passports, signed authorization, and revocation chains for sovereign agent identity. | Do not bind Goni to a draft protocol before local identity requirements are specified. | `primary-source verified` |
| AgentHook | https://agenthook.org/ | Runtime evidence and lifecycle events | Study common event shapes for tool calls, approvals, denials, policy decisions, and evidence bundles. | Do not replace Goni receipts with generic observability events. | `primary-source verified` |
| Microsoft Agent Governance Toolkit | https://github.com/microsoft/agent-governance-toolkit | Runtime policy, sandboxing, governance | Compare zero-trust identity, policy checks, and runtime security controls. | Do not import enterprise cloud assumptions into Goni's personal local-first trust model. | `primary-source verified` |
| AgentMint | https://github.com/aniketh-maddipati/agentmint | Signed receipts and runtime authorization | Study minimal Ed25519-style action proof patterns. | Do not treat signature presence as receipt completeness. | `primary-source verified` |
| Sanna | https://sanna.dev/ | Constitution-as-code and governance receipts | Study policy-before-action and cryptographic governance receipt framing. | Do not import policy branding without a Goni policy language and conformance tests. | `primary-source verified` |
| Delegation Receipt Protocol | https://datatracker.ietf.org/doc/draft-nelson-agent-delegation-receipts/ | Signed delegation receipts | Study authorization object fields, scope boundaries, time windows, and model-state commitments. | Do not claim standards compliance while the draft is unstable. | `primary-source verified` |
| Authenticated Delegation and Authorized AI Agents | https://arxiv.org/abs/2501.09674 | Authenticated delegated authority | Use IAM-style authenticated delegation as an academic anchor for bounded authority. | Do not treat legal authority as solved by authentication alone. | `primary-source verified` |
