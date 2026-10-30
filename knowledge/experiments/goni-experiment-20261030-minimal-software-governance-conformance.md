---
id: GONI-EXPERIMENT-20261030-MINIMAL-SOFTWARE-GOVERNANCE-CONFORMANCE
title: Minimal software and governance conformance evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Evaluate whether smaller permanent software implementations preserve authorization, recoverability, policy legibility, and useful task completion at lower measured total system cost.
domains:
- repository
- system
aliases: []
relations:
- type: tests
  target: GONI-DECISION-20261030-MINIMAL-SUFFICIENT-SOFTWARE
- type: tests
  target: GONI-PRINCIPLE-20261030-EXPLICIT-NAMING
- type: tests
  target: GONI-PRINCIPLE-20261030-CAPABILITY-GROWTH-WITH-BOUNDED-AUTHORITY
- type: tests
  target: GONI-SPEC-20261030-VISUAL-GOVERNANCE-TRACKS
- type: tests
  target: GONI-SPEC-20261030-NATURAL-LANGUAGE-POLICY-TRANSLATION
sources: []
artifacts: []
uncertainty: Planned evaluation only; no implementation result, reliability guarantee, or comparative performance finding is asserted.
legacy: []
---

# Minimal software and governance conformance evaluation

## Hypothesis

A minimal deterministic authority substrate with model-mediated planning may achieve the same required security and reliability as a more elaborate permanent application implementation at lower total cost for some workloads. This must be demonstrated rather than assumed.

## Representative scenarios

- A permitted research task completes autonomously with a reconstructable receipt.
- An external message outside the granted recipient scope is denied despite a low model-assigned risk score.
- A capability expires or is revoked before an effect, and the action is denied.
- Ambiguous natural-language instructions produce an inactive, reviewable policy proposal.
- A newly generated tool or workflow inherits existing scopes without expanding authority.
- Model unavailability suspends adaptive work while preserving canonical policy, state, and revocation.
- A human interprets the visual tracks and identifies denied, approval-required, and review-after actions correctly.
- Different contributors and models interpret descriptive terminology consistently.

## Evaluation method

Compare a minimal candidate with a conventional implementation using matched workloads, tools, permissions, inputs, models, and resource budgets where applicable.

Measure correctness, unauthorized effects, receipt completeness, reconstruction, revocation latency, human comprehension, interruptions, inference and compute cost, code and dependency footprint, maintenance, and recovery work.

Report the tested boundary, implementation commit, workload, sample size, model, hardware, failures, limitations, and reproducibility steps. The safety-critical target is zero unauthorized effects within the tested threat model. Improvements in line count alone are insufficient.

This is an experiment specification. Implementation evidence and comparative conclusions remain outstanding.
