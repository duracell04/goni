---
id: GONI-IMAP-AF201DE9A613
title: 1. Role in the system
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The Visual Runtime: executes policy-approved visual workflow templates, loads only approved visual model bundle IDs, accepts source assets, masks, controls, and references by stable refs, returns output hashes and intermediate artifact refs, reports runtime capabilities and utilization to scheduling policy, exposes enough workflow provenance for receipts and rollback.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/visual-runtime.md
  heading: 1. Role in the system
  revision: 4d6a56dfeb55430356f9e72b203b5df766df28e8
---

# 1. Role in the system

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Role in the system

The Visual Runtime:

- executes policy-approved visual workflow templates,
- loads only approved visual model bundle IDs,
- accepts source assets, masks, controls, and references by stable refs,
- returns output hashes and intermediate artifact refs,
- reports runtime capabilities and utilization to scheduling policy,
- exposes enough workflow provenance for receipts and rollback.

The Goni kernel owns Work Orders, Done Contracts, asset permissions,
capability tokens, model eligibility, receipts, memory updates, and approval
corridors. The runtime receives authority decisions; it does not create them.
