---
id: GONI-IMAP-512A79A3E16F
title: 3.5 Delegation previews and action cards
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The dashboard may show the kernel''s reconstruction panel for gated actions: Goal Done Assumptions Risk Question These views are read-only projections of Work Orders, receipts, and policy state.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/dashboard-concepts.md
  heading: 3.5 Delegation previews and action cards
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 3.5 Delegation previews and action cards

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.5 Delegation previews and action cards

The dashboard may show the kernel's reconstruction panel for gated actions:

- Goal
- Done
- Assumptions
- Risk
- Question

These views are read-only projections of Work Orders, receipts, and policy
state. The dashboard may approve, reject, or cancel through public APIs, but
it may not rewrite the underlying reconstruction locally.

> **Invariant UI-4 (kernel-backed reconstruction)**
> Any approval card or delegation preview shown in the dashboard must be
> derived from kernel-backed Work Order / receipt state. No UI-only
> reconstruction objects are allowed.

---
