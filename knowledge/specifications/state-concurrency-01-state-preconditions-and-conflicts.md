---
id: STATE-CONCURRENCY-01
title: State Preconditions and Concurrency
type: specification
status: draft
implementation_state: specified_only
proposition: Consequential execution must revalidate the state predicates and resource versions on which authorization depended immediately before commit; stale or conflicting state requires observation and replanning rather than blind execution.
domains:
- kernel
- state
- tools
- system
aliases:
- optimistic agent concurrency
- stale-state protection
relations:
- type: depends_on
  target: HARNESS-RUNTIME-01
- type: depends_on
  target: TOOL-01
- type: refines
  target: SPEC-TXN-01
sources: []
artifacts: []
uncertainty: Concrete version primitives vary by filesystem, database, application API, remote service, and GUI surface. This contract specifies semantic precondition behavior rather than one universal compare-and-swap mechanism.
legacy: []
---

# State Preconditions and Concurrency

Goni distinguishes the state used to plan and authorize an action from the state that exists when the action is about to commit.

A proposal may be correct relative to an observed snapshot and still be invalid at execution time because another user, process, agent, service, or remote system changed the relevant resource.

## Canonical rule

For an effectful action with commit-time predicates:

```text
authorize against observed state
-> prepare
-> reacquire or validate current state
-> evaluate commit-time predicates
-> execute only when predicates still hold
```

Execution is permitted only when every required commit-time predicate holds in the current authoritative or task-appropriate observed state.

## Required references

Effectful proposals should carry, where the target surface supports them:

- `observed_state_snapshot_id`;
- `resource_version_refs`;
- `precondition_refs`;
- `precondition_freshness_policy`;
- `conflict_scope`; and
- `reconciliation_strategy_ref` for operations whose outcome can become ambiguous.

A resource version may be an API ETag, row/version number, repository commit, file identity plus metadata, application revision, calendar event version, transaction state, or another adapter-specific concurrency token.

## Outcomes

Commit-time state validation distinguishes at least:

- `ready`: required predicates still hold;
- `stale_state`: relevant state changed and the proposal must be reconsidered;
- `conflict`: another effect is incompatible with the pending operation;
- `unobservable`: required state cannot be established with sufficient assurance.

These outcomes do not authorize the harness to repeat or weaken the original preconditions.

## Recovery

The default recovery path is:

```text
STALE_STATE | CONFLICT | UNOBSERVABLE
-> OBSERVE
-> update task/belief state
-> REPLAN or ESCALATE
-> obtain fresh authorization when the material action changes
```

Idempotency prevents duplicate execution of the same logical operation; concurrency control prevents a once-valid operation from committing against a materially different world. The two mechanisms are complementary.

## Multi-resource effects

For actions spanning several resources, the ToolManifest or transaction contract should state the atomicity boundary. When the external systems cannot provide a single atomic commit, Goni must preserve which preconditions were checked, which effects committed, and which compensation or reconciliation obligations remain.

## Invariant

Authorization never freezes the external world.

A capability or policy decision permits an operation only while the state predicates that formed part of that decision remain satisfied.
