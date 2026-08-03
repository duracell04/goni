---
id: GONI-SYNTHESIS-8A2FFAA7A328
title: Local-first runtime and agent OS substrate
type: synthesis
status: draft
implementation_state: specified_only
proposition: '| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence | | OpenJarvis | https://github.com/open-jarvis/OpenJarvis | Local-first runtime and resource evaluation | Use FLOPs, energy, latency, and cost as first-class local-agent metrics.'
domains:
- research
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/references/gonios-research-neighbor-map.md
  heading: Local-first runtime and agent OS substrate
  revision: 08e1061f9ab1e1a95e22a924fdc9970e0585851b
---

# Local-first runtime and agent OS substrate

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Local-first runtime and agent OS substrate

| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| OpenJarvis | https://github.com/open-jarvis/OpenJarvis | Local-first runtime and resource evaluation | Use FLOPs, energy, latency, and cost as first-class local-agent metrics. | Do not collapse resource evaluation into a generic assistant UX claim. | `primary-source verified` |
| AIOS | https://arxiv.org/abs/2403.16971 | Agent OS scheduling, context, memory, storage, access control | Compare Goni's scheduler, context management, and tool governance against an agent-OS architecture. | Do not inherit "OS" language without Goni-specific mediation, receipts, and local-first boundaries. | `primary-source verified` |
| AIOS repository | https://github.com/agiresearch/AIOS | Agent SDK/runtime reference | Inspect implementation boundaries for agent scheduling and runtime services. | Do not treat the SDK as Goni's kernel or trusted computing base. | `primary-source verified` |
| OS-Copilot / FRIDAY | https://arxiv.org/abs/2402.07456 | Computer-use action execution and self-improvement loop | Study OS-level task execution across web, terminal, files, and applications. | Do not import autonomous action patterns without Goni capability tokens and receipts. | `primary-source verified` |
| OS-Copilot repository | https://github.com/OS-Copilot/OS-Copilot | Generalist computer agent implementation | Compare action surfaces and tool APIs. | Do not use it as evidence for safe delegated authority. | `primary-source verified` |
