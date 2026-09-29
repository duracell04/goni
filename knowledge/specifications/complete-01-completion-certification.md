---
id: COMPLETE-01
title: Completion Certification
type: specification
status: draft
implementation_state: specified_only
proposition: Goni may represent consequential delegated work as completed only after qualifying post-action evidence satisfies its DoneContract; intermediate tool, workflow, provider, or control-plane success is insufficient unless the DoneContract explicitly defines that state as the objective.
domains:
- agent
- kernel
- specs
- system
aliases:
- completion-certification
relations:
- type: depends_on
  target: GONI-SPEC-59E3DFC23CC8
- type: depends_on
  target: GONI-SPEC-F37FC6D98E05
- type: depends_on
  target: REC-01
- type: refines
  target: GONI-SPEC-423A86A50102
sources: []
artifacts: []
uncertainty: COMPLETE-01 is specified-only. Concrete verifier implementations, evidence-independence rules, domain-specific postconditions, and acceptable residual false-completion rates require implementation and evaluation.
legacy: []
---

# COMPLETE-01 - Completion Certification

> Status boundary: this is a specified-only contract. Enforcement language
> describes intended conformance behavior rather than observed implementation.

## 1. Purpose

COMPLETE-01 governs when Goni may convert an attempted or executed action into a
claim that the principal's delegated objective is complete.

> **An executed action is not a completed task.**

The authority to change external state and the authority to certify what state
now exists are distinct kernel decisions.

## 2. Completion state machine

Consequential WorkOrders SHOULD preserve:

```text
proposed -> authorized -> executed -> observed -> verified -> certified_complete
```

- **proposed:** a plan exists without execution authority.
- **authorized:** policy and capability checks permit the bounded action.
- **executed:** the action path reports that its operation was carried out or
  accepted at that interface.
- **observed:** post-action evidence about the resulting state has been
  collected.
- **verified:** the DoneContract predicates have been evaluated against
  qualifying evidence.
- **certified_complete:** every required completion predicate passes and the
  kernel permits a completion claim.

The lifecycle also permits `failed`, `unknown`, and
`verification_incomplete`. The last state means execution may have succeeded
while the DoneContract cannot yet be evaluated to the required standard.

## 3. Proxy-goal separation

Signals produced by an action provider describe that provider's state unless
the DoneContract says otherwise. A deployment platform reporting `READY`, an
API returning an identifier, or a command exiting successfully may be valuable
action evidence without proving the higher-level user objective.

```text
ToolSuccess(T) != Done(T)
```

For WorkOrder `W` with DoneContract `D`:

```text
Done(W) := all required predicates in D are supported by qualifying observed evidence
```

## 4. Post-action verification

Verification SHOULD use a read-after-write or otherwise discriminating
observation path appropriate to the delegated objective.

Where material and practical, the verification path SHOULD be informationally
independent from the action provider's self-report. Independence requires
evidence that tests the target postcondition rather than merely repeating the
signal whose meaning is in question.

Examples include public HTTP or browser observation after deployment,
provider-visible readback after message submission, event readback after
calendar creation, reopening or validating a written file, inspecting the
target ref after a merge, and querying resulting state after a database change.

When qualifying verification is unavailable, Goni must report the strongest
state actually established rather than manufacture certainty.

## 5. Completion claims

Words such as `sent`, `booked`, `deployed`, `fixed`, `submitted`,
`published`, or `backed up` are certified-state claims when they represent
external state material to the user's next action.

A model may generate hypotheses or summaries about state, but it MUST NOT
promote an inferred or provider-reported state to `certified_complete` without
the kernel-visible verification required by the DoneContract.

When verification remains incomplete, the user-facing result should preserve
that boundary, for example: "The deployment operation completed, but the public
application state remains unverified."

## 6. Receipts and memory

The completion decision MUST remain reconstructable through REC-01 evidence.
The receipt chain should preserve the action outcome, observations,
verification results, DoneContract reference, and resulting completion state.

Memory updates that depend on a consequential outcome SHOULD preserve the
certified state or unresolved verification boundary rather than storing an
unverified completion claim as fact.

## 7. Conformance invariants

A conforming implementation must preserve at least:

- zero silent promotion from action success to `certified_complete`;
- zero `certified_complete` state without a satisfiable DoneContract;
- explicit preservation of `unknown` and `verification_incomplete`;
- reconstructable evidence for consequential completion certification.
