---
id: EXP-INTENT-EFFECT-01
title: 'End-to-End Delegation Cost and Scope-Control Benchmark'
type: experiment
status: draft
implementation_state: not_applicable
proposition: 'Measure whether cross-layer coordination and marginal-value scope control improve correct, authorized, externally verified task completion per unit of system cost versus matched existing-runtime baselines.'
domains:
- agent
- hardware
- research
- system
aliases: []
relations:
- type: tests
  target: INTENT-EFFECT-01
- type: tests
  target: AGENT-NET-VALUE-01
- type: depends_on
  target: METRICS-01
sources:
- SRC-XIE2024-OSWORLD
- SRC-HORVITZ1999-MIXED-INITIATIVE
artifacts: []
uncertainty: 'This is a planned experiment, not implementation or performance evidence. Candidate gains, thresholds, and costs are unmeasured.'
legacy: []
---

# End-to-End Delegation Cost and Scope-Control Benchmark

## Questions

Does cross-plane coordination lower cost per correct, authorized, externally verified task? Does a marginal-value admission rule reduce optional subtasks, corrections, and human interruptions without reducing task correctness or mandated verification?

## Controlled comparisons

Use synthetic scheduling-email and file-organization tasks with seeded inputs, explicit DoneContracts, simulated external effects, and independently checked expected outcomes.

1. Existing GONI architecture with ordinary component-level optimization and unchanged policy.
2. An experimental cross-plane coordination strategy under the same model, safety requirements, and task budget.
3. A scope-control ablation varying optional-subtask admission and stopping discipline while keeping the required deliverable fixed.

Where equivalent target operations exist, separately compare typed API calls and GUI-mediated execution. Deliberately unmediated security comparisons belong only in isolated synthetic fixtures.

Record model and runtime versions, hardware, prompt/task fixture, baseline configuration, context size, concurrency, seeds, retries, warm/cold state, and declared resource ceilings. Start with deterministic smoke tests and small samples; expand only where measurements justify more compute.

## Measurements

- Correct, authorized, externally verified completion rate, including explicit failure and timeout denominators.
- End-to-end latency (median/p95 when supported by sample size), queueing, inference, tool, and verification contributions.
- Compute, network, energy where measured, human work, retries, and recovery costs per verified task.
- Unauthorized effects, false authorization, prohibited egress, and verification fidelity.
- Generated optional subtasks, out-of-scope changes, steps after DoneContract satisfaction, and avoidable approval prompts.

Distinguish instrumented measurements from estimates, including long-term maintenance cost.

## Falsification and publication

Predeclare quality and safety non-inferiority criteria, a material-benefit threshold, and resource and stopping budgets. Report null results and regressions. Reuse EXP-INF-PLACE-01, EVID-SEC-01, EVID-AUTON-01, and EVID-ENF-01 where their experimental boundaries apply.

Executable tests and measurements belong in duracell04/goni-prototype-lab. Results become canonical evidence only through pinned revisions, stated fixtures, and documented limitations.
