---
id: PLACEMENT-EVAL-01
title: Capability-qualified placement evaluation
type: experiment
status: draft
implementation_state: specified_only
proposition: A matched evaluation should compare capability-qualified routes using
  complete execution, evidence, transfer, failure, latency, and energy costs under
  identical hard constraints.
domains:
- evaluation
- scheduler
- hardware
aliases: []
relations:
- type: tests
  target: COMP-REG-01
- type: tests
  target: HCOMP-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Supported platforms, cost instrumentation, workload suite, and comparison
  thresholds remain to be selected.
legacy: []
---

# Capability-qualified placement evaluation

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

Compare local, owner-mesh, and authorized cooperative routes for the same named
computation/evidence profile. Vary memory capacity, data/model locality,
bandwidth, thermal headroom, stale/false capability advertisements, unavailable
proof backends, failures, deadlines, and disclosure constraints.

Measure eligibility decisions, objective estimates versus actual total costs,
transfer/evidence overhead, latency, energy where measured, retries, and result
acceptance. Keep unsupported counters explicitly unknown. Record objective units
and weights, capability observations, runtime/full implementation revisions,
hardware configurations, and execution/evidence receipts. Compare against a named
execution-time-only baseline and a permitted-local baseline. Set statistical
and utility thresholds before execution; the conformance target is zero placement
outside declared hard constraints within the tested boundary.
