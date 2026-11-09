---
id: OPSOV-EXP-01
title: "Adversarial dependency and continuity tests"
type: experiment
status: draft
implementation_state: specified_only
proposition: "Matched adversity experiments should measure whether additional dependency controls improve principal control beyond local inference and kernel-mediated tool permissions."
domains:
- research
- security
- system
aliases: []
relations:
- type: tests
  target: OPSOV-PRINC-01
- type: tests
  target: OPSOV-SPEC-01
- type: depends_on
  target: GONI-DECISION-7CD481DA15B7
sources:
- SRC-NIST-SP800-160V2R1
- SRC-NIST-SP800-161R1-UPD1
- SRC-TORRES-ARIAS-IN-TOTO-2019
artifacts: []
uncertainty: "Planned protocol and untested hypothesis; no runtime performance or assurance results are claimed."
legacy: []
---

# Adversarial dependency and continuity tests

**H1 (unverified).** Local inference and kernel authorization alone leave measurable failure modes when critical infrastructure or providers retain unmediated third-party control. Additional independently controlled provenance, offline identity, recovery and evidence protections are hypothesized to reduce such failures at an operational cost.

Compare baseline **B** (local inference and tool mediation) with extended **S** (B plus enhanced independence and recovery mechanisms) on matched hardware, tasks, budgets, threat capabilities and observation windows. Use local retrieval, private inference, policy-authorized tools, receipt export and backup recovery as declared critical workflows; distinguish inherently external services.

Test OPSOV-A01–A08 through mocked provider outages, revoked accounts, restricted network routes, poisoned updates, damaged backups and ultimately hardware failures. Record functional continuity and task correctness, unauthorized observable egress, integrity violations, update rejection, receipt tamper detection given independent anchors, restore success, measured RTO/RPO, operator intervention, compute/energy, latency and maintenance burden.

Publish per-threat results with adverse outcomes and residual unknowns. An aggregate sovereignty score would need a separate validated weighting model; use multidimensional measures initially.

**Current status:** experiment specification, intended for later execution in `goni-prototype-lab`; tested observations require pinned evidence records.
