# VISION
Status: Specified only / roadmap

This document defines product-level framing for Goni OS.
Normative behavior still lives in `blueprint/30-specs/`.

## 1. Category stance

Goni is not a Linux distribution business.
Goni is a governed delegation runtime that runs on top of a Linux substrate.

Working shorthand:
- Linux is the execution base.
- Goni is the policy and governance layer above that base.

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
- non-bypassable capability mediation,
- tool isolation and default-deny egress control,
- receipts and provenance for side effects,
- local-first durable memory with explicit sync policy,
- extension verification and revocation workflows,
- predictable updates, rollback, and incident operations.

## 4. Product thesis

Goni should be framed as:
- "agent OS governance layer running on Linux",
not:
- "another Linux flavor with agent features."

The product promise is governed execution, not kernel novelty.

## 5. North-star outcomes

- Users can delegate meaningful work without ambient authority.
- Every mediated action is reviewable through receipts.
- Extension velocity does not break security posture.
- Local-first behavior remains useful without cloud dependency.
- Operators can reason about risk, rollback, and accountability.
