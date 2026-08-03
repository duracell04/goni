---
id: GONI-SPEC-463A1E62F73C
title: 9. Auditing semantics (receipts)
type: specification
status: draft
implementation_state: specified_only
proposition: Receipts prove policy application, not truth of content or external identity.
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
  heading: 9. Auditing semantics (receipts)
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 9. Auditing semantics (receipts)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. Auditing semantics (receipts)

Receipts prove policy application, not truth of content or external identity.
They are evidence that the Gate enforced the declared constraints at time of
egress. Receipts intentionally do not store raw payloads by default.

Recommended receipt fields (stored in AuditRecords.provenance):

- timestamp, duration_ms, bytes_up/down
- agent_id, capability_token_id
- policy_hash, state_snapshot_id
- purpose, classification, route_used
- budgets applied and budget exhaustion flags
- retention_mode and redaction mode applied

Anonymous Mode defaults to minimal retention and omits destinations/URLs unless
the user explicitly opts into verbose logging.
