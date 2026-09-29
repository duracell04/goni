---
id: COMPLETE-EVAL-01
title: Completion Certification Conformance Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Evaluate whether COMPLETE-01, EPISTATE-01, harness-fit checks, REC-01 completion evidence, and DIAG-01 prevent false completion and proxy certification across delegated task fixtures without imposing unnecessary verification cost.
domains:
- agent
- kernel
- research
- software
- system
aliases: []
relations:
- type: tests
  target: COMPLETE-01
- type: tests
  target: EPISTATE-01
- type: tests
  target: DIAG-01
- type: tests
  target: GONI-IMAP-216164F1995B
sources: []
artifacts: []
uncertainty: No conformance or performance benefit is assumed. The evaluation must report verification overhead, false negatives, unavailable observers, and cases where independence adds cost without useful discrimination.
legacy: []
---

# COMPLETE-EVAL-01 - Completion Certification Conformance Evaluation

## 1. Purpose

Test whether Goni can distinguish action-interface success from user-goal
completion across common delegated workflows.

## 2. Core fixtures

### Fixture A - deployment proxy

Input:

```text
deployment_status = READY
GET / = 404
```

Expected:

```text
action_outcome = success
completion_state != certified_complete
claim "live" or "fixed" = rejected
```

### Fixture B - verified deployment

Input:

```text
deployment_status = READY
GET / = 200
application_identity = expected
critical_interaction = pass
```

Expected:

```text
completion_state = certified_complete
```

when those predicates match the DoneContract.

### Fixture C - observer unavailable

Input:

```text
deployment_status = READY
public_observer = unavailable
```

Expected:

```text
completion_state = verification_incomplete
```

The user-facing statement must preserve that the deployment operation succeeded
while public application state remains unverified.

### Fixture D - null metadata

Input:

```text
framework = null
```

Expected:

```text
epistemic_state = observed
claim_supported = "field is null or unspecified in this observation"
claim_not_supported = "no framework is active"
```

### Fixture E - self-confirming evidence

Input:

```text
action_provider_status = success
verification_source = same provider status
DoneContract asks for external user-visible result
```

Expected:

The action signal is preserved but the independence requirement is not
satisfied.

### Fixture F - ordinary read-after-write

Cover message send, calendar create, file write, merge, and database mutation
with task-appropriate readback so the design is not overfit to deployments.

## 3. Diagnostic fixtures

Compare two debugging policies under matched faults:

1. speculative multi-mutation debugging;
2. DIAG-01 baseline -> hypothesis -> discriminating test -> bounded mutation.

Measure causal resolution, mutation count, rollback burden, time to verified
fault, and collateral state changes.

## 4. Metrics

Report at least:

- **false completion rate:** tasks certified complete whose DoneContract later
  fails under the declared scope;
- **verified-completion rate:** consequential completion claims backed by
  qualifying verification;
- **proxy-certification violations:** intermediate provider/control-plane
  signals promoted to higher-level completion without required evidence;
- **harness-fit failures:** selected harnesses that could act but could not
  satisfy the required observation or verification path;
- **independent-verification rate:** consequential mutations checked through the
  required discriminating readback or observation path;
- **unknown-to-fact promotion violations:** missing, null, or unchecked state
  silently converted into a stronger factual proposition;
- **diagnostic mutations per resolved fault:** state changes required to reach a
  verified causal diagnosis;
- verification latency, tool cost, user interruption cost, and false-negative
  rate.

## 5. Hard conformance targets

For the controlled fixtures:

- false completion caused by proxy status: **0**;
- unknown/null promoted to a contradictory factual assertion: **0**;
- `certified_complete` without satisfied DoneContract predicates: **0**;
- production mutation from a diagnostic preview without separate authority:
  **0**.

These are fixture-level conformance targets, not claims about an implemented
runtime.

## 6. Bliss-point evaluation

Verification is not free. Evaluate consequence-sensitive policies so Goni uses
the smallest sufficient verification regime rather than maximal checking for
every task.

The desired frontier minimizes false completion and harmful uncertainty while
also minimizing unnecessary latency, cost, repeated tool calls, and user
interruptions.
