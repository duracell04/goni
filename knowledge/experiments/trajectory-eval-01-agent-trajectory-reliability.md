---
id: TRAJECTORY-EVAL-01
title: Agent trajectory reliability evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Goni Lab should evaluate delegated agents over complete, repeated state-changing trajectories using external postconditions, hidden-state recovery, constraint retention, clarification behavior, verification, and recovery rather than relying on single-run model judgments.
domains:
- research
- evaluation
- agents
aliases:
- trajectory reliability
- long-horizon agent evaluation
relations:
- type: tests
  target: GONI-THESIS-D2E69CC012F4
sources:
- SRC-LU2024-TOOLSANDBOX
- SRC-YAO2024-TAUBENCH
- SRC-JIMENEZ2023-SWEBENCH
- SRC-YUAN2026-OSWORLD2
artifacts: []
uncertainty: This is a proposed Goni Lab evaluation family. Scenario fixtures, thresholds, and acceptable reliability levels remain to be implemented and calibrated.
legacy: []
---

# Agent trajectory reliability evaluation

Goni Lab should evaluate a delegated system as a **trajectory through changing state**, not only as a sequence of plausible messages.

## Core evaluation dimensions

### 1. External-state success

Where an objective postcondition exists, success should be computed from the resulting environment:

- expected database state,
- file or repository state,
- calendar or communication state,
- test and compiler results,
- policy state,
- receipt and provenance state.

The generator's own claim of success is not a sufficient evaluator.

### 2. Repeated-run reliability

Each representative task should be repeated under controlled seeds or equivalent stochastic variation. Report:

- single-run success,
- repeated success using a pass^k-style measure where suitable,
- failure-mode distribution,
- variance in tool-call count, latency, and cost,
- policy and receipt consistency across runs.

### 3. Partial-observability handling

Fixtures should include:

- hidden or implicit state,
- stale memory,
- contradictory observations,
- state that changes during execution,
- insufficient information,
- information arriving after the task starts.

Measure whether the agent gathers evidence, updates belief/task state, or escalates appropriately.

### 4. Constraint retention

Long workflows should include persistent constraints that remain relevant hundreds of steps later. Record:

- forgotten constraints,
- unauthorized shortcuts,
- context or memory substitution errors,
- goal drift,
- silent assumption changes.

### 5. Clarification quality

Human escalation is successful behavior when information is decision-relevant and unavailable. Measure:

- unnecessary-question rate,
- missed-clarification rate,
- whether the question is the smallest decisive one,
- whether safe progress occurs before asking when possible.

### 6. Verification and recovery

Measure whether the system:

- checks preconditions,
- verifies postconditions,
- detects partial failure,
- retries idempotently where safe,
- rolls back or compensates where supported,
- replans from observed state instead of repeating the same failed action.

## Runtime-monitor shape

A strong trajectory should approximate:

```text
check preconditions
-> act
-> observe actual result
-> verify invariants/postconditions
-> update task and belief state
-> continue / replan / escalate / stop
```

This experiment family should be applied to Goni's own delegation workflows rather than importing benchmark scores as a proxy for sovereign-operation quality.
