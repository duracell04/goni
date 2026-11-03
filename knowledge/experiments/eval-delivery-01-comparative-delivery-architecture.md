---
id: EVAL-DELIVERY-01
title: "Comparative evaluation of delivery reliability and complexity"
type: experiment
status: draft
implementation_state: not_applicable
proposition: "Measure whether a smaller CI and deployment system meets GONI assurance requirements at lower total operating cost than more elaborate alternatives."
domains:
- software
- validation
- research
- delivery
aliases: []
relations:
- type: tests
  target: DELIVERY-MIN-01
- type: tests
  target: OPS-CONTRACT-01
- type: tests
  target: CI-ASSURANCE-01
sources:
- SRC-BARDSIRI2026-GHA-LANGUAGE
- SRC-BARDSIRI2026-STUCK-ON-RED
- SRC-MASTOURI2026-AGENT-OPS
artifacts: []
uncertainty: "Proposed evaluation only. No measured GONI results or causal conclusions are claimed."
legacy: []
---

# Comparative evaluation of delivery reliability and complexity

## Research question

Among deployment and CI designs meeting the same functional and authority contracts, which design has the lowest total maintenance, execution, and incident-response burden?

## Comparison

Use the same synthetic application changes, controlled inputs, and mandatory conformance requirements across (A) current reference delivery, (B) a consolidated, risk-targeted pipeline with semantic checks, and (C) a more elaborate alternative only where a distinct capability is justified. Document incomparable requirements.

## Faults and observations

Inject invalid canonical relations, stale catalogue projections, missing persistent mounts, incompatible image/dependency versions, failed receipt writes, process crashes, container replacement, interrupted approvals, uncertain third-party actions, network outages, and denied capabilities. Measure consequential defects detected and missed, false failure rates, time to diagnosis and recovery, repeat failure streaks, duplicate effects, workflow minutes, resource cost, operational interventions, and review burden.

## Acceptance

Pre-register mandatory security, durability, and recovery thresholds. Reject any candidate that violates a hard guarantee, regardless of speed or cost. Among conformant designs, prefer the least burdensome candidate within statistical and operational uncertainty. Pinned experimental results, not this plan, may later support promoted blueprint claims.
