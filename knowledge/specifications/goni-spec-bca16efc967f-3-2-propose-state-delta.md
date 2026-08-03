---
id: GONI-SPEC-BCA16EFC967F
title: 3.2 `propose_state_delta`
type: specification
status: draft
implementation_state: specified_only
proposition: propose_state_delta(tx_id, delta_ref, provenance_ref) -> proposal_ref No user-visible state change is committed by this call alone.
domains:
- agent
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md
  heading: 3.2 `propose_state_delta`
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.2 `propose_state_delta`

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.2 `propose_state_delta`

`propose_state_delta(tx_id, delta_ref, provenance_ref) -> proposal_ref`

No user-visible state change is committed by this call alone.
