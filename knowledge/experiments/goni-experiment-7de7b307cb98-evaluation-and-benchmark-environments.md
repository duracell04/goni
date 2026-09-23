---
id: GONI-EXPERIMENT-7DE7B307CB98
title: Evaluation and benchmark environments
type: experiment
status: draft
implementation_state: not_applicable
proposition: '| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence | | OSWorld | https://os-world.github.io/ | Desktop/computer-use benchmark | Study reproducible multimodal desktop tasks for future LifeBench-style evaluation.'
domains:
- research
aliases: []
relations: []
sources:
- SRC-XIE2024-OSWORLD
- SRC-LIU2023-AGENTBENCH
- SRC-LU2024-TOOLSANDBOX
- SRC-YAO2024-TAUBENCH
- SRC-JIMENEZ2023-SWEBENCH
- SRC-YUAN2026-OSWORLD2
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/references/gonios-research-neighbor-map.md
  heading: Evaluation and benchmark environments
  revision: 08e1061f9ab1e1a95e22a924fdc9970e0585851b
---

# Evaluation and benchmark environments

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Evaluation and benchmark environments

| Project | Primary source | GoniOS layer mapping | What Goni should learn | What Goni should not import | Confidence |
| --- | --- | --- | --- | --- | --- |
| OSWorld | https://os-world.github.io/ | Desktop/computer-use benchmark | Study reproducible multimodal desktop tasks for future LifeBench-style evaluation. | Do not treat OSWorld task success as sufficient for personal-memory delegation quality. | `primary-source verified` |
| OSWorld repository | https://github.com/xlang-ai/OSWorld | Benchmark harness | Inspect environment construction and task packaging. | Do not import real-user traces into public fixtures. | `primary-source verified` |
| WebArena | https://github.com/web-arena-x/webarena | Self-hostable web-agent environment | Use realistic reproducible websites for delegated web-action tests. | Do not expose private accounts or real browsing workflows in public tests. | `primary-source verified` |
| WorkArena | https://github.com/ServiceNow/WorkArena | Enterprise knowledge-work benchmark | Study office-work task decomposition for Action Cards and Daily Briefs. | Do not assume enterprise SaaS tasks represent personal sovereign workflows. | `primary-source verified` |
| Mind2Web | https://osu-nlp-group.github.io/Mind2Web/ | Web-agent dataset and generalization benchmark | Study task representation and website generalization labels. | Do not import non-synthetic personal workflows into Goni fixtures. | `primary-source verified` |
| AgentBench | https://openreview.net/forum?id=zAdUB0aCTQ | Multi-environment agent benchmark | Study multi-domain task coverage and agent evaluation reporting. | Do not use broad agent scores as a proxy for Goni memory, receipt, or privacy quality. | `primary-source verified` |

| ToolSandbox | https://arxiv.org/abs/2408.04682 | Stateful tool-use benchmark | Evaluate implicit state dependencies, insufficient-information cases, and trajectory milestones. | Do not treat tool competence as proof of Goni authority or privacy guarantees. | `primary-source verified` |
| tau-bench | https://arxiv.org/abs/2406.12045 | Tool-agent-user and policy benchmark | Compare final environment state with goal state and measure repeated-run reliability using pass^k-style metrics. | Do not reduce Goni reliability to one aggregate benchmark metric. | `primary-source verified` |
| SWE-bench | https://arxiv.org/abs/2310.06770 | Executable software-engineering benchmark | Prefer executable postconditions over model self-assessment where deterministic verification exists. | Do not assume code-test verification transfers unchanged to social or ambiguous workflows. | `primary-source verified` |
| OSWorld2.0 | https://arxiv.org/abs/2606.29537 | Long-horizon computer-use benchmark | Test persistent constraints, hidden-state recovery, changing information, clarification, and verification across hundreds of actions. | Do not equate desktop completion with sovereign delegation quality. | `primary-source verified` |
