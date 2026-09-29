---
id: DIAG-01
title: Controlled Diagnostic Mutation
type: specification
status: draft
implementation_state: specified_only
proposition: When causal uncertainty is material, Goni SHOULD preserve a stable baseline, prefer discriminating observation before mutation, state the active hypothesis, bound diagnostic mutations, and preserve rollback or preview isolation so diagnosis does not destroy the subject being diagnosed.
domains:
- agent
- kernel
- software
- specs
- system
aliases:
- controlled-diagnostics
relations:
- type: depends_on
  target: EPISTATE-01
- type: depends_on
  target: GONI-IMAP-216164F1995B
sources: []
artifacts: []
uncertainty: DIAG-01 is a specified diagnostic discipline. Mutation budgets, preview requirements, and escalation thresholds must vary by task consequence and require evaluation.
legacy: []
---

# DIAG-01 - Controlled Diagnostic Mutation

> Status boundary: this is a specified-only contract. Enforcement language
> describes intended conformance behavior rather than observed implementation.

## 1. Purpose

Debugging is itself delegated work. When Goni changes a system while trying to
understand it, each mutation can erase evidence, introduce new causes, or move
the target state.

The default diagnostic sequence is:

```text
baseline
-> observation
-> hypothesis
-> discriminating test
-> bounded mutation
-> observation
-> belief update
```

The governing rule is:

> **When causal uncertainty is high, increase observation before increasing mutation.**

## 2. Diagnostic WorkOrder fields

A diagnostic WorkOrder SHOULD preserve, directly or by stable reference:

- `baseline_ref`: the known state from which diagnosis begins;
- `hypotheses`: active causal explanations and their epistemic state;
- `discriminating_tests`: checks that would separate competing explanations;
- `mutation_budget`: a consequence-sensitive bound on state changes;
- `preview_or_isolation`: whether a non-production environment is available;
- `rollback_ref`: rollback, revert, compensation, or restoration path;
- `production_mutation_policy`: conditions under which production may change.

## 3. Read-before-write discipline

Where a read-only or reversible observation can discriminate among material
hypotheses, Goni SHOULD prefer it before changing the target system.

A mutation SHOULD have a stated causal purpose: what hypothesis it tests or
what verified fault it repairs. Multiple unrelated speculative changes SHOULD
not be bundled merely because they are individually plausible.

## 4. Stable experimental subject

Diagnostics SHOULD preserve a stable experimental subject whenever practical.

If branch configuration, deployment mode, workflow policy, routing, project
metadata, and application code are all changed during one unresolved diagnosis,
subsequent observations lose causal resolution.

Preview, staging, branch isolation, snapshots, or reversible patches SHOULD be
used when their marginal cost is justified by blast radius and uncertainty.

## 5. Production discipline

Diagnostic work MUST respect ordinary authority policy.

A diagnostic branch, pull request, preview deployment, or test artifact SHOULD
not move production state merely because it exists to gather evidence.
Production mutation requires its own authorization and DoneContract.

## 6. Receipt and learning

Diagnostic receipts SHOULD preserve the hypothesis being tested, the
observation gathered, the mutation made, the resulting evidence, and whether
the hypothesis gained or lost support.

Learning systems may use diagnostic outcomes to improve harness policy, but
failed diagnosis must not be rewritten as successful causal knowledge.
