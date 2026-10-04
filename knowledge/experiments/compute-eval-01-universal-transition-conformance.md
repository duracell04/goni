---
id: COMPUTE-EVAL-01
title: Universal transition conformance evaluation
type: experiment
status: draft
implementation_state: specified_only
proposition: A matched evaluation should test whether the composed computation, execution,
  evidence, and receipt contracts preserve their declared acceptance boundaries across
  execution profiles.
domains:
- compute
- evaluation
aliases: []
relations:
- type: tests
  target: COMP-01
- type: tests
  target: EXEC-01
- type: tests
  target: EVID-01
- type: tests
  target: RESULT-01
- type: tests
  target: DIST-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Supported backend and evidence profiles, fixtures, and acceptance thresholds
  require a pinned experiment implementation.
legacy: []
---

# Universal transition conformance evaluation

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

## Question and comparison

Compare a local baseline with local, owner-mesh, and permitted remote execution of
the same named computation specification. Run exact, tolerance-based, and
stochastic profiles only where an implementation declares support.

## Planned cases

- Exact program execution: valid result, changed input/program/starting state,
  and a reused computation under a fresh execution contract.
- Approximate execution: named comparison norm, reference and tolerance; values
  immediately inside and outside the declared acceptance boundary.
- Stochastic execution: a named sampling/randomness contract and statistically
  defined acceptance test, distinguished from bitwise replay.
- Evidence: substituted verifier or evidence policy, correlated replicas,
  stale attestation, unsupported evidence mechanisms, and meter claims outside
  the proof's scope.
- Completion: missing output data, cancellation, stale starting state, revoked
  authority, and replayed settlement authorization.

## Measurements and artifacts

Record program/runtime and full implementation commit, execution identity,
evidence type and assumptions, accept/reject outcome, false acceptance/rejection,
latency, evidence cost, verification cost, transfer cost, and meter assurance.
Produce scope-pinned receipts and case results. A conforming boundary should
reject each deliberately invalid claim under the selected profile. These are
planned cases; successful execution will require separate evidence records.
