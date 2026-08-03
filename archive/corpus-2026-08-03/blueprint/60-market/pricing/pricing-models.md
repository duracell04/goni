# Pricing Models for Agent OS (No Numeric Assumptions)

Status: Specified only / roadmap
Last updated: 2026-02-20

This document captures business model patterns for open, local-first agent
platforms. It is strategy guidance only, not a launch plan.

## 1. Core framing

The durable model is usually:
- open runtime or open distro as adoption layer,
- paid trust and operations layer above it.

For Goni, the kernel/base image is not the primary monetized unit.
The monetizable layer is governed execution and accountability.

## 2. What users pay for (repeated pattern)

Common paid layers in open infrastructure products:
- managed control plane and fleet operations,
- security updates and long support windows,
- enterprise hardening and policy packs,
- compliance reporting and audit exports,
- incident response and support SLAs,
- certified integrations and compatibility guarantees.

## 3. Agent-specific paid layer

For agent ecosystems, demand clusters around:
- governance and policy assurance,
- extension verification and revocation,
- supply-chain trust (signing, provenance, attestations),
- auditable execution records and forensic support,
- safe managed distribution of updates and connectors.

This aligns with Goni's design direction:
- capabilities + non-bypassable mediation,
- receipts + provenance,
- network gate and constrained egress,
- safe extension lifecycle.

## 4. Anti-patterns to avoid

- Selling "another Linux distro" as the core differentiator.
- Competing on extension volume without verification depth.
- Monetizing raw token throughput while underinvesting in trust controls.
- Making safety optional add-ons instead of default product behavior.

## 5. Evaluation rubric for model fit

A commercialization track is valid only if it improves at least one:
1. measurable risk reduction,
2. operator accountability,
3. upgrade and incident response reliability.

If it does not improve one of those, it should be treated as secondary.

## 6. Positioning line (internal)

"Open base, paid trust: governance, verification, managed operations, and
accountable execution."
