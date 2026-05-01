# VISION
Status: Specified only / roadmap

This document defines product-level framing for Goni OS.
Normative behavior still lives in `blueprint/30-specs/`.

## 1. Category stance

Goni is not a Linux distribution business.
Goni is a delegation OS and bounded digital twin that runs on top of a Linux
substrate.

Working shorthand:
- Linux is the execution base.
- Goni is the delegation and governance layer above that base.
- The stack makes Goni possible, but delegation is the product.

Terminology guardrail:
- macOS is Unix-based (Darwin/XNU), not Linux.
- Keep comparisons technically accurate when positioning Goni.

## 2. Why Linux is the substrate

Linux is chosen for practical reasons:
- default runtime for servers and cloud workloads,
- broad embedded and appliance footprint,
- mature isolation and security primitives,
- favorable economics for long-lived appliances,
- strong tooling around packaging, observability, and operations.

This is a base-layer choice, not the product moat.

## 3. What is hard and valuable

A distro alone is rarely the product. The durable value is:
- reconstruction of vague human intent into Work Orders and Done Contracts,
- non-bypassable capability mediation,
- tool isolation and default-deny egress control,
- receipts and provenance for side effects,
- local-first durable memory with explicit sync policy,
- extension verification and revocation workflows,
- predictable updates, rollback, and incident operations.

## 4. Product thesis

Goni should be framed as:
- "local sovereign delegation OS running on Linux",
not:
- "another Linux flavor with agent features."

The product promise is reduced digital friction through governed delegation,
not kernel novelty. Users express intent naturally; Goni reconstructs the goal,
defines done, assembles context, selects tools and models, acts within autonomy
corridors, tracks open loops, and emits receipts.

Digital twin framing means bounded representative, not clone. Goni knows enough
about the principal to act competently inside defined corridors, and to stop
when judgment, permission, or risk requires escalation.

## 5. North-star outcomes

- Users can delegate meaningful work without ambient authority.
- Users interact less with computers, web workflows, documents, and online
  services because Goni handles clerical digital work.
- Every mediated action is reviewable through receipts.
- Extension velocity does not break security posture.
- Local-first behavior remains useful without cloud dependency.
- Operators can reason about risk, rollback, and accountability.
