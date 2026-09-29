---
id: EPISTATE-01
title: Epistemic State Typing
type: specification
status: draft
implementation_state: specified_only
proposition: Material system-state claims used for consequential delegation MUST preserve whether they are observed, inferred, hypothesized, verified, or certified; missing, null, or unavailable evidence must remain unknown or unspecified unless separate evidence establishes a stronger proposition.
domains:
- agent
- kernel
- memory
- specs
- system
aliases:
- epistemic-state-typing
relations:
- type: depends_on
  target: COMPLETE-01
- type: refines
  target: GONI-SPEC-A742123055E0
sources: []
artifacts: []
uncertainty: EPISTATE-01 specifies logical epistemic classes, not a shipping schema. Domain-specific confidence calibration, evidence-strength rules, and compact runtime representations require implementation and evaluation.
legacy: []
---

# EPISTATE-01 - Epistemic State Typing

> Status boundary: this is a specified-only contract. Enforcement language
> describes intended conformance behavior rather than observed implementation.

## 1. Purpose

Goni must preserve the difference between what the system directly encountered,
what a model concluded from that evidence, what remains a possibility, what has
been checked, and what the kernel is prepared to represent as established.

For material system-state claims, the minimum logical classes are:

- **observed:** an artifact, response, state, or event was directly obtained
  within the declared observation scope;
- **inferred:** a conclusion was derived from observations or other evidence;
- **hypothesized:** a candidate explanation or proposition remains unresolved;
- **verified:** a declared predicate was tested against qualifying evidence;
- **certified:** the applicable governance contract permits the proposition to
  be represented as established for the current scope and purpose.

Implementations MAY use richer states, but they MUST preserve these semantic
boundaries where collapsing them could affect authority, completion, memory, or
a user's next action.

## 2. Unknown is a first-class state

Missing metadata, a null field, an unavailable observation, a failed lookup, or
an unchecked surface establishes an evidence boundary. It does not establish
the opposite proposition.

```text
unknown != false
missing != absent
unobserved != disproved
```

For example, `framework = null` establishes that the inspected field is null
or unspecified in that observation. It does not by itself establish that no
framework is active elsewhere in the system.

## 3. Evidence and inference must remain separable

The system SHOULD preserve enough provenance to reconstruct the observation,
the inference derived from it, assumptions required by the inference, any
verification performed, and the contract that permitted certification.

A model-generated interpretation MUST NOT silently overwrite its source
observation. Repeated inference from the same evidence lineage is not new
observation and does not become independent confirmation merely through
repetition.

## 4. Consequential use

Before a material inference is used to widen authority, certify task completion,
promote durable memory to fact, justify a consequential external action, or make
a strong negative claim, the applicable policy or contract must determine
whether the current epistemic state is sufficient.

COMPLETE-01 requires verified DoneContract predicates before
`certified_complete`. Memory promotion remains governed by the applicable
memory contract.

## 5. User-facing claims

User-facing wording SHOULD express the strongest state actually established.

- observed: "The deployment API reports READY."
- inferred: "That suggests the artifact may be available."
- unknown: "The public route has not been checked."
- verified: "A public request returned the expected application."
- certified: "The deployment is complete under this DoneContract."

The system must not gain apparent certainty merely because fluent language can
compress these distinct states into one sentence.
