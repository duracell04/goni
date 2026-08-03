---
id: GONI-SYNTHESIS-2F73035842DF
title: 2. Agent loop patterns
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Shunyu Yao** Why track: ReAct-style reasoning/acting loops and related agentic planning patterns.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/inspiration.md
  heading: 2. Agent loop patterns
  revision: 7ea8b5d90372661a27969ad2680c98b3c75f000a
---

# 2. Agent loop patterns

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2. Agent loop patterns

- **Shunyu Yao**
  - Why track: ReAct-style reasoning/acting loops and related agentic planning
    patterns.
  - Goni angle: useful for the `intent -> plan -> tool` loop, even when Goni
    constrains that loop through policy and mediation.

- **Graham Neubig / CMU ecosystem**
  - Why track: practical work on agent orchestration and evaluation.
  - Goni angle: useful for benchmarking whether agents can actually complete
    delegated work.

- **Jason Wei**
  - Why track: prompting, reasoning, and evaluation discipline around model
    behavior.
  - Goni angle: useful as a cautionary baseline for where raw reasoning patterns
    help and where Goni needs harder control-plane structure.
