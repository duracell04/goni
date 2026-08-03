---
id: GONI-SPEC-50622E0EB363
title: 9. Tool And Receipt Interface
type: specification
status: draft
implementation_state: specified_only
proposition: Robot skill calls are TOOL-01 tool calls.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/embodied-robot-control-plane.md
  heading: 9. Tool And Receipt Interface
  revision: 9e24971edf51dd4248752851642dc996837a82ab
---

# 9. Tool And Receipt Interface

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. Tool And Receipt Interface

Robot skill calls are TOOL-01 tool calls. They must preserve:

- `boundary_basis`,
- `work_order_id`,
- `done_contract_hash`,
- `autonomy_mode`,
- `risk_score`,
- `risk_basis`,
- `capability_token_id`,
- `idempotency_key` for mutating calls,
- `undo_strategy_ref` or explicit no-rollback metadata,
- `task_class` with a `robot.*` value when robot behavior affects the action.

Receipts for robot-mediated actions may include `robot_basis`, analogous to
`visual_basis`. `robot_basis` extends REC-01 and is not a separate receipt
type. It stores compact refs and hashes rather than raw sensor streams, full
video, raw audio, private household maps, unrestricted telemetry, or
unbounded transcripts.

Robot adapters are external execution substrates. Their logs can support
evidence, but they cannot be the only terminal record of a mediated physical
effect.
