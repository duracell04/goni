---
id: GONI-IMAP-A8159C5AB617
title: Plane 𝒦 – Control (metadata only)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Concepts: requests, scheduling state.'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/30-plane-contracts.md
  heading: Plane 𝒦 – Control (metadata only)
  revision: dcbe5931107b72f6a6af295e9e1b943accb6a2f9
---

# Plane 𝒦 – Control (metadata only)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Plane 𝒦 – Control (metadata only)
- Concepts: requests, scheduling state.
- Tables: Requests, Tasks, AuditRecords, CapabilityTokens, RedactionProfiles, RedactionEvents, AgentManifests.
- Allowed FK targets: `request_id` referenced by 𝒳 (Prompts/Contexts) and ℰ (LlmCalls); `task_id` referenced by ℰ spans.
- Forbidden: `LargeUtf8` fields; raw text never stored here.
