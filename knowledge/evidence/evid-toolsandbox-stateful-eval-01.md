---
id: EVID-TOOLSANDBOX-STATEFUL-EVAL-01
title: 'Source claim: ToolSandbox evaluates stateful tool trajectories'
type: evidence
status: draft
implementation_state: not_applicable
proposition: ToolSandbox evaluates conversational tool use with stateful execution, implicit dependencies between tools, insufficient-information cases, and milestone checks over complete trajectories.
domains:
- research
- evaluation
- tools
aliases: []
relations:
- type: supports
  target: TRAJECTORY-EVAL-01
sources:
- SRC-LU2024-TOOLSANDBOX
artifacts: []
uncertainty: ToolSandbox covers tool-use competence and does not by itself measure Goni-specific receipt, privacy, or authority guarantees.
legacy: []
---

# Source claim: ToolSandbox evaluates stateful tool trajectories

ToolSandbox moves evaluation beyond single-turn function calling by making intermediate and final state transitions part of the benchmark.

For Goni, this supports evaluating hidden and implicit state dependencies, insufficient information, and recovery across an interaction trajectory.
