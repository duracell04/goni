---
id: GONI-PRINCIPLE-20261030-CAPABILITY-GROWTH-WITH-BOUNDED-AUTHORITY
title: Capability growth within human-defined authority
type: principle
status: draft
implementation_state: specified_only
proposition: Goni may discover and improve capabilities autonomously within approved mandates, while changes to effective authority require a separate explicit human-governed authorization transition.
domains:
- agent
- kernel
- product
aliases: []
relations:
- type: refines
  target: GONI-DECISION-9AF49466170D
- type: depends_on
  target: GONI-DECISION-20261030-ARTIFICIAL-INTELLIGENCE-NATIVE-FOUNDATION
- type: depends_on
  target: GONI-DECISION-5A5B3B8A34CA
sources: []
artifacts: []
uncertainty: Specific capability-promotion thresholds and approval workflows remain to be verified against representative tasks and adversarial scenarios.
legacy: []
---

# Capability growth within human-defined authority

Goni's adaptive intelligence may learn procedures, generate code, choose tools, combine agents, and propose new ways of accomplishing a task. These changes expand **capability**. Permission to perform consequential effects is a separate, human-governed **authority** decision.

## Core boundary

- New skills and generated workflows inherit only the effective authority of their current mandate and capability scopes.
- A model's confidence, a successful test, a new model, or a newly discovered tool cannot independently grant access to additional data, recipients, accounts, networks, budgets, or mutation rights.
- An authority expansion requires an explicit policy change approved by an authorized human principal, with a reviewable difference, new policy version, and receipt.
- Authority checks resolve against canonical kernel-owned state at the time of effect, including expiry and revocation.
- The human may authorize a bounded policy corridor once, allowing routine work inside that corridor without repeated confirmations.
- Generated code and newly assembled agent workflows use the same execution mediation as built-in tools; their novelty provides no special exemption.

## Human-legible growth

The governance interface should show both the capabilities Goni has gained and the exact scope of authority those capabilities may exercise. The AI may propose new tracks or wider boundaries, but the proposed expansion remains inactive until the governing transition is authorized and recorded.

This principle separates open-ended problem-solving potential from tightly scoped operational power.
