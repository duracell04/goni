---
id: GONI-PRINCIPLE-E216CB231421
title: 6. Security invariants
type: principle
status: draft
implementation_state: specified_only
proposition: No external egress without Gate mediation.
domains:
- network
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/network-gate-and-anonymity.md
  heading: 6. Security invariants
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 6. Security invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Security invariants

1. No external egress without Gate mediation.
2. No ambient internet for agents/tools; only the Gate has outbound privileges.
3. Overlay use requires explicit capability and policy approval.
4. Every egress emits an audit receipt tied to agent_id, policy_hash, and
   state_snapshot_id.
5. Anonymous Mode receipts are non-deanonymizing by default; logging is treated
   as a side channel and minimized unless explicitly enabled.
6. Extracted screen/app context cannot be sent to a remote model or service
   without both extraction permission and Network Gate approval.
