---
id: GONI-PRINCIPLE-329BE7E070FA
title: 2. v1.0 Table Set (ships in binary)
type: principle
status: draft
implementation_state: specified_only
proposition: 'We ship the following canonical tables in v1.0: Docs Chunks Embeddings Prompts ContextItems Requests Tasks AuditRecords CapabilityTokens RedactionProfiles RedactionEvents AgentManifests StateSnapshots StateDeltas LatentSummaries MemoryEntries LlmCalls PlatformSignals PlatformCapabilities Metrics Specified-only visual extensions queued for a schema DSL/prototype slice:'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/10-axioms-and-planes.md
  heading: 2. v1.0 Table Set (ships in binary)
  revision: 43a497b2a7deb59e07ad598a7c0496fbc9dc3cbe
---

# 2. v1.0 Table Set (ships in binary)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. v1.0 Table Set (ships in binary)

We ship the following canonical tables in v1.0:

1. Docs
2. Chunks
3. Embeddings
4. Prompts
5. ContextItems
6. Requests
7. Tasks
8. AuditRecords
9. CapabilityTokens
10. RedactionProfiles
11. RedactionEvents
12. AgentManifests
13. StateSnapshots
14. StateDeltas
15. LatentSummaries
16. MemoryEntries
17. LlmCalls
18. PlatformSignals
19. PlatformCapabilities
20. Metrics

Specified-only visual extensions queued for a schema DSL/prototype slice:
VisualAssets and VisualAssetDerivations. They do not ship in the current binary
table set until added to `53-schema-dsl-and-macros.md` and the executable DSL.

Any new canonical table must be added to the schema DSL (see
`53-schema-dsl-and-macros.md`) and documented in `51-schemas-mvp.md`.
