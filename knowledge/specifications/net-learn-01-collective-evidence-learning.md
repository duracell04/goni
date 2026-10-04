---
id: NET-LEARN-01
title: Collective evidence learning
type: specification
status: draft
implementation_state: specified_only
proposition: GONI nodes may contribute policy-approved, provenance-bearing evidence
  to generate evaluated candidate improvements whose adoption remains locally authorized
  and reviewable.
domains:
- learning
- network
- privacy
aliases: []
relations:
- type: depends_on
  target: NET-COMP-01
- type: depends_on
  target: EPI-SEP-01
- type: depends_on
  target: EPI-CTRL-01
- type: depends_on
  target: GONI-SPEC-C237E3663D04
- type: refines
  target: GONI-SPEC-7254B180A7F4
- type: refines
  target: GONI-SPEC-C4655EDC9751
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Evidence schemas, independence estimation, poisoning resistance, privacy
  composition, candidate efficacy, and contribution incentives remain unvalidated.
legacy: []
---

# Collective evidence learning

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

## Evidence export

\[
Experience_i\rightarrow Evidence_i\rightarrow Aggregate
\rightarrow CandidatePatch\rightarrow Eval\rightarrow OwnerAdoption.
\]

Experience remains owner-governed. A conforming contributor MUST derive a
purpose-scoped evidence record through NET-COMP-01's export boundary before
sharing it. The record identifies its claim, observation or derivation class,
scope, method, implementation/model revision where relevant, evidence lineage,
uncertainty, and permitted use/disclosure. Receipt access follows REC-01's
owner-facing privacy boundary. Synthetic hypotheses retain their epistemic class.

## Aggregation

A conforming aggregation policy MUST state its accepted evidence classes,
provenance/lineage assumptions, deduplication, contribution weighting,
contradiction handling, poisoning threat model, and privacy assumptions.
Repeated observations with a shared material origin retain their dependency;
unresolved independence is recorded as unknown. Quantity or agreement alone
does not establish corroboration or permission. EPI-SEP-01 governs evidence versus
hypothesis separation and EPI-CTRL-01 supplies lineage-aware support handling.

## Candidate and adoption boundary

A candidate patch references a target and base revision, proposed change,
supporting/challenging evidence, limitations, and an evaluation plan. It may
propose a harness, retrieval, model/adaptor, or workflow change within existing
adaptation contracts. It enters the existing review, regression, rollout and
rollback mechanisms for that target.

Before adoption, a conforming principal MUST assess compatibility with current
state and policy, evaluate the candidate against a named baseline and required
regressions, and record the authority and scoped result. An unreviewed or failed
candidate remains a proposal or is rejected. Each owner controls adoption and
revocation. Network participation supplies neither installation authority nor
an automatic change to canonical memory, mandates, permissions, or base weights.

Blueprint-node changes use this repository's editorial/governance and commit
contracts. Planned evaluations remain experiments; observed results receive
separate evidence records with their full implementation and test boundaries.
