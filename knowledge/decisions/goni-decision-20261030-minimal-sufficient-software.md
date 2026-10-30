---
id: GONI-DECISION-20261030-MINIMAL-SUFFICIENT-SOFTWARE
title: Minimal Sufficient Software
type: decision
status: draft
implementation_state: specified_only
proposition: Goni chooses the smallest permanent software substrate sufficient for required functionality, safety, reliability, human authority, and accountability, evaluated by total system complexity.
domains:
- software
- system
aliases: []
relations:
- type: refines
  target: GONI-DECISION-42DF477FE7FB
- type: depends_on
  target: GONI-PRINCIPLE-3CE61BB5417A
sources: []
artifacts: []
uncertainty: The relative cost of model-mediated and conventional implementations is workload-dependent and requires measurement; this is a design rule, not an empirically proven optimal implementation.
legacy: []
---

# Minimal Sufficient Software

## Decision

Goni uses the smallest permanent software substrate that can meet the required functional, performance, security, authority, state-integrity, recovery, and accountability guarantees.

Artificial intelligence handles interpretation, planning, adaptation, and task-specific procedures where it is effective. Deterministic code handles authorization, resource boundaries, durable state, execution mediation, receipts, and computations that are simpler or more reliable as conventional software.

The selection objective is **minimum total system complexity among implementations meeting the same required guarantees**, rather than minimum source-line count.

Total complexity includes permanent code, packages, dependencies, services, configuration, model inference, prompting, verification, human review, monitoring, latency, incident recovery, maintenance, and operating cost. A longer deterministic implementation is preferable when it is demonstrably simpler or safer at the system level.

## Implementation discipline

1. Reuse an existing kernel contract or interface when it satisfies the need.
2. Prefer a direct module, declarative contract, or single process until measured constraints justify an additional abstraction, process, dependency, or service.
3. Use adaptable model behavior for variable work and stable code for repeated operations when the latter reduces total cost and uncertainty.
4. Keep the trusted authorization and effect boundary small enough to inspect and test independently of model behavior.
5. Justify every architectural addition by the specific requirement it satisfies, its measurable benefit, its complete operational cost, and its rollback path.
6. Simplify the underlying workflow before automating it. Preserve hidden functions such as exception handling, human trust, and recovery when they remain necessary.

## Consequences

- Architectural proposals compare the simplest viable alternative with the proposed design.
- Each added permanent component carries a named requirement and evidence plan.
- Existing correct, inexpensive deterministic code remains eligible even when a model could regenerate it.
- Goni's capabilities may expand without requiring one dedicated permanent application implementation for every new workflow.

This decision refines the single-node simplicity objective and operates within sovereign modularity. It neither weakens existing kernel invariants nor claims that a smaller codebase is automatically secure.
