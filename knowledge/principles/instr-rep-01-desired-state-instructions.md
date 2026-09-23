---
id: INSTR-REP-01
title: Desired-State Instruction Representation
type: principle
status: draft
implementation_state: specified_only
proposition: Goni should express trajectory instructions primarily as the desired valid state or action path, while retaining explicit prohibitions where the forbidden state itself is a material security, legal, authority, or scope boundary.
domains:
- harness
- instructions
- models
aliases:
- trajectory versus boundary instructions
relations:
- type: refines
  target: MSC-01
- type: refines
  target: DELEG-INT-01
- type: depends_on
  target: TRUST-INPUT-01
sources:
- SRC-JANG2022-NEGATED-PROMPTS
artifacts: []
uncertainty: The cited work studies specific model families and negated prompts rather than establishing a universal law for all modern models. The principle is therefore a representation preference subject to empirical evaluation, not a ban on negation.
legacy: []
---

# Desired-State Instruction Representation

Goni distinguishes **trajectory instructions** from **boundary instructions**.

## Trajectory instructions

Trajectory instructions guide the model toward a desired valid state or execution path. Where the affirmative representation carries the intended meaning by itself, Goni should prefer it because it concentrates context on the state the model should construct.

Prefer:

```text
Persistent application state uses the canonical datastore.
Extend that datastore when new persistent state is required.
```

when those statements fully express the contract, rather than relying on a rejected alternative as the primary representation.

## Boundary instructions

Boundary instructions communicate a forbidden state whose prohibition is itself operationally important. Explicit negative constraints remain appropriate for:

- security boundaries;
- legal or compliance restrictions;
- destructive or irreversible operations;
- authority limits;
- privacy and secret-handling constraints;
- explicit non-goals; and
- ambiguity where a plausible invalid interpretation must be excluded.

A strong boundary formulation should pair the prohibited state with the valid alternative where useful:

```text
Secrets remain in the governed secret store.
Repository configuration contains secret identifiers or handles only.
```

## Engineering rule

For each instruction, ask:

1. Is the rejected state a realistic interpretation that must be excluded?
2. Does naming it add decision-relevant information?
3. Can the desired state be represented directly and more compactly?
4. Is the constraint better enforced by software, schema, capability, sandbox, or test?
5. Does the instruction belong globally, locally, or only in the current WorkOrder?

This principle minimizes unnecessary representational branches while preserving explicit hard boundaries.

## Evidence boundary

Research on negated prompts reports substantial degradation and inverse-scaling behavior in the evaluated model families. That finding motivates reducing unnecessary dependence on negation. It does not establish that language models categorically cannot represent or follow negative constraints.

Goni should therefore evaluate instruction formulations against actual deployment profiles rather than encode a universal linguistic rule.
