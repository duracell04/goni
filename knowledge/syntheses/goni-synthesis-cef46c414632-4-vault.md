---
id: GONI-SYNTHESIS-CEF46C414632
title: 4) Vault
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Purpose: local system of record for durable memory and provenance pointers.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/10-primitives.md
  heading: 4) Vault
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 4) Vault

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4) Vault
- Purpose: local system of record for durable memory and provenance pointers.
- Contract anchors: `software/vault/README.md`, `30-specs/sync-policies.md`.
- Core invariant: long-term memory is local-first; cloud paths use minimized context.
- Metrics: retrieval p95, ingest latency, citation coverage.
