---
id: GONI-THESIS-3234CF1EA4B0
title: 9. Local-First Computing and Network Governance
type: thesis
status: draft
implementation_state: specified_only
proposition: Goni's local-first thesis is not simply that models run on local hardware.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/05-sovereign-delegation-os-thesis.md
  heading: 9. Local-First Computing and Network Governance
  revision: c48031be8a4ca8ba57323dccb803c8dcdaab65dc
---

# 9. Local-First Computing and Network Governance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. Local-First Computing and Network Governance

Goni's local-first thesis is not simply that models run on local
hardware. Local-first means that core functions are intended to be computable
using local state and local compute; network access is capability-scoped;
external calls are optional, transparent, budgeted, and receipt-linked; and
remote model usage passes through an explicit network gate in the target
architecture.

This is important because "local AI" can become marketing if network behavior
remains implicit. A serious sovereign AI system treats network egress as an
effect requiring governance. Private memory is not intended to leak to cloud
services by accident. Remote inference does not occur merely because a model router
found it convenient. The network gate is therefore a core authority boundary,
not an implementation detail.

The simplest early version of this principle is strict: default-deny egress as
the design intent, one mediated egress API, explicit policy modes, and receipts
for outbound calls. More complex modes can emerge later, but the first proof
needs to show that local-first behavior is testable rather than aspirational. The
network gate design is specified in
[network-gate-and-anonymity.md](/blueprint/30-specs/network-gate-and-anonymity.md).
