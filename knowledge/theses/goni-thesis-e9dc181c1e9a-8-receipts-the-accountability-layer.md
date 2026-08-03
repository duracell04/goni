---
id: GONI-THESIS-E9DC181C1E9A
title: '8. Receipts: The Accountability Layer'
type: thesis
status: draft
implementation_state: specified_only
proposition: Receipts are the mechanism by which Goni makes action reconstructable.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/05-sovereign-delegation-os-thesis.md
  heading: '8. Receipts: The Accountability Layer'
  revision: c48031be8a4ca8ba57323dccb803c8dcdaab65dc
---

# 8. Receipts: The Accountability Layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Receipts: The Accountability Layer

Receipts are the mechanism by which Goni makes action reconstructable. In a
delegated AI system, the key question is not only whether the system succeeded,
but whether its behavior can be explained after the fact. A receipt helps
answer:

- Why did the system act?
- What data did it use?
- Which policy allowed or denied the action?
- Which model or tool was involved?
- What changed?
- What risk level was assigned?
- Was network egress involved?
- Could the action be replayed, audited, compensated, or rolled back?

This makes receipts fundamentally different from logs. Logs are operational
traces. Receipts are governance artifacts. They are minimal but sufficient
records of consequential action, designed to preserve evidence, provenance,
policy context, and accountability. The receipt contract is described in
[receipts.md](/blueprint/30-specs/receipts.md).

Receipts also serve a trust-building function. Users do not need to inspect
every internal operation, but they need confidence that consequential behavior
is reconstructable. Goni's receipt model therefore turns invisible agentic
behavior into accountable delegation.

Because receipts may refer to sensitive activity, the design also distinguishes
receipt completeness from raw data retention. Receipts are intended to store
refs, hashes, bounded summaries, and replay metadata by default, not raw private
content unless a governing policy and data class permit it.
