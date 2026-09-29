---
id: TIME-01
title: Temporal Constraints and Deadline Authority
type: specification
status: draft
implementation_state: specified_only
proposition: Goni MUST distinguish operative deadlines from internal target dates and MUST represent who has authority to change a date; changing a target MUST NOT silently alter the operative deadline it protects.
domains:
- specs
- control-plane
- scheduling
aliases:
- temporal-authority
- deadline-authority
relations:
- type: refines
  target: GONI-SPEC-78027C2E0EED
  note: Refines the generic JobSpec deadline field by separating scheduler-facing deadlines from real-world temporal constraints and their change authority.
sources: []
artifacts: []
uncertainty: The human-facing FIXED, COMMITTED, and TARGET labels are a Goni operational taxonomy. The underlying authority dimensions are normative; the labels are derived presentation semantics rather than claims of standardized academic terminology.
legacy: []
---

# TIME-01 - Temporal Constraints and Deadline Authority

> Status boundary: this specification is design-bearing and `specified_only`. It defines intended semantics, not an implemented or verified runtime mechanism.

## 1. Purpose

A timestamp alone is insufficient for delegated scheduling.

Goni must know not only **when** something is expected, but also **what role the date plays, who controls it, and what is required to change it**. This prevents a scheduling optimizer from treating an externally imposed filing deadline, a renegotiable client commitment, and a self-chosen completion target as equivalent movable dates.

The core rule is:

> **A date is a temporal constraint plus an authority relationship.**

## 2. Canonical dimensions

A temporal constraint MUST preserve the following semantic dimensions.

### 2.1 Date role

`date_role`:

- `operative_deadline` - a date whose miss has an external, policy, contractual, institutional, regulatory, social, or otherwise operative consequence;
- `target` - an internally selected desired completion date used for planning, pacing, or self-regulation;
- `unknown` - the system has identified a potentially consequential date but cannot yet classify its role safely.

### 2.2 Control authority

`control_authority`:

- `external` - the principal cannot unilaterally change the operative date;
- `shared` - changing the date requires agreement, coordination, approval, waiver, amendment, or another actor's participation;
- `internal` - the principal can change the date unilaterally;
- `unknown` - authority has not been established with sufficient confidence.

### 2.3 Change mechanism

`change_mechanism`:

- `external_exception` - change requires an extension, waiver, authority decision, formal exception, or equivalent external process;
- `renegotiation` - change requires coordination or agreement with another actor or incurs commitment cost;
- `unilateral` - the principal may change the date directly;
- `unknown` - the valid change path is not yet established.

These dimensions are stored independently because a source category such as `client`, `university`, or `government` does not by itself determine how a date may be changed.

## 3. Derived human-facing classes

The user-facing taxonomy is derived from the underlying authority state.

### FIXED deadline

A temporal constraint is presented as **FIXED** when it is an `operative_deadline` and the principal lacks unilateral authority to change it.

Typical representation:

```yaml
date_role: operative_deadline
control_authority: external
change_mechanism: external_exception
derived_class: fixed
```

"Fixed" means **not unilaterally movable by the principal**. It does not claim that extensions or exceptions are impossible.

### COMMITTED deadline

A temporal constraint is presented as **COMMITTED** when it is an operative date bound to another actor or commitment mechanism and moving it requires renegotiation, coordination, approval, or meaningful commitment cost.

Typical representation:

```yaml
date_role: operative_deadline
control_authority: shared
change_mechanism: renegotiation
derived_class: committed
```

A committed deadline may be highly consequential even when it is technically movable.

### TARGET date

A temporal constraint is presented as **TARGET** when it is an internally controlled desired completion date.

Typical representation:

```yaml
date_role: target
control_authority: internal
change_mechanism: unilateral
derived_class: target
```

Targets are planning instruments, not substitutes for the operative deadlines they protect.

### UNKNOWN

`unknown` is an internal safety state, not a normal user-facing deadline class.

If Goni cannot establish whether a discovered date is movable, it MUST NOT silently treat the date as a target. The system may retrieve more evidence, proceed conservatively, or raise a clarification interrupt when the classification materially changes safe delegated execution.

## 4. Source and consequence are orthogonal metadata

A temporal constraint SHOULD separately preserve:

- `source_kind`, such as `government | university | client | project | personal | contract | other`;
- `consequence_kind`, such as `legal | financial | contractual | reputational | operational | self_regulatory | mixed | unknown`;
- source/provenance references sufficient to reconstruct where the date came from.

These fields describe the origin and stakes of the date. They do not replace `control_authority`.

## 5. Operative deadline versus scheduler deadline

The existing JobSpec `deadline` is scheduler-facing: it constrains when a Goni job should receive service.

A real-world temporal constraint is a separate domain fact.

For example:

```text
University submission deadline: 31 October      -> operative deadline
Internal completion target:    28 October      -> target
Goni review job:                27 October 08:00 -> scheduler deadline
```

Implementations MUST NOT overload one timestamp to represent all three concepts.

Scheduler deadlines may be derived from temporal constraints and policy, but they do not replace the underlying constraint.

## 6. Protected targets and safety margin

A target MAY reference the operative constraint it protects through a stable relation such as `protects_constraint_ref`.

Given:

```text
target_at = 28 October
operative_deadline_at = 31 October
```

the system may derive:

```text
safety_margin = operative_deadline_at - target_at = 3 days
```

The safety margin is a derived interval, not a fourth deadline class.

When either linked date changes, the derived safety margin MUST be recomputed.

## 7. Authority-sensitive behavior

Temporal classification affects what Goni may do.

### TARGET

Within an active autonomy corridor, Goni may reschedule an internal target, create a replacement target, rebalance work around it, or reduce its safety margin, subject to policy.

### COMMITTED

Goni may detect breach risk, propose a revised date, prepare negotiation or communication, and act only within the authority granted for changing or renegotiating that commitment.

Ordinary scheduling authority alone is insufficient to mutate a committed deadline.

### FIXED

Goni treats a fixed deadline as a protected external boundary.

It may reprioritize movable work around the deadline, increase urgency, surface breach risk, prepare required work, or initiate an authorized extension/exception workflow. It MUST NOT resolve overload by simply moving the fixed deadline.

### UNKNOWN

Goni must preserve uncertainty until sufficient evidence or authority exists to classify the date.

## 8. Temporal authority invariants

- Changing a `target` MUST NOT silently mutate a linked `operative_deadline`.
- A `fixed` deadline MUST NOT be rescheduled through ordinary scheduling authority.
- A `committed` deadline MUST NOT be changed without the relevant renegotiation, coordination, approval, or delegated authority.
- An `unknown` temporal constraint MUST NOT be silently treated as freely movable.
- Scheduler-facing deadlines MUST remain distinguishable from real-world temporal constraints.
- Temporal decisions that materially affect delegated execution MUST remain reconstructable through stable constraint references and provenance.

## 9. Relationship to Goni authority architecture

TIME-01 applies Goni's broader separation of intelligence and authority to time.

A model may infer that a schedule would improve if a date moved. That inference does not establish permission to move the date. The Control Plane must evaluate the temporal constraint's authority state and applicable policy before scheduling cognition becomes an external or canonical state change.
