---
id: GONI-SYNTHESIS-3BAF6D1DACDF
title: Runtime verification and assurance
type: synthesis
status: draft
implementation_state: specified_only
proposition: '| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence | | AgentGuard | https://arxiv.org/abs/2509.23864 | Runtime verification and agent assurance | Study formal events, monitorable safety properties, and MDP-style assurance framing.'
domains:
- research
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/references/gonios-research-neighbor-map.md
  heading: Runtime verification and assurance
  revision: 08e1061f9ab1e1a95e22a924fdc9970e0585851b
---

# Runtime verification and assurance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Runtime verification and assurance

| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| AgentGuard | https://arxiv.org/abs/2509.23864 | Runtime verification and agent assurance | Study formal events, monitorable safety properties, and MDP-style assurance framing. | Do not call Goni a POMDP or MDP implementation before states, actions, transitions, rewards, and evaluations are formalized. | `primary-source verified` |
| SWE-agent | https://github.com/SWE-agent/SWE-agent | Agent-computer interface design | Study how interface design changes agent task success and repair loops. | Do not generalize software-engineering agent results to personal delegation without LifeBench evidence. | `primary-source verified` |
| OpenHands | https://github.com/All-Hands-AI/OpenHands | Mature open-source software-agent platform | Study sandbox/workspace patterns and action trace ergonomics. | Do not treat coding-agent infrastructure as a personal memory OS. | `primary-source verified` |
