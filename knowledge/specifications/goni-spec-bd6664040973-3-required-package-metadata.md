---
id: GONI-SPEC-BD6664040973
title: 3. Required package metadata
type: specification
status: draft
implementation_state: specified_only
proposition: 'Every package entry MUST include: package_id (stable identifier) version (semver or monotonic) publisher_id artifact_digest (content hash) signature (publisher signature over digest) manifest (AGENT-MANIFEST-01 compliant) capability_declarations (requested tools + scopes) sandbox_profile_id (SANDBOX-01 mapping) update_policy (auto, prompt, or pinned)'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agents/agent-store.md
  heading: 3. Required package metadata
  revision: 17d060e8ac309d5a25765a07f3a00da85d0739e1
---

# 3. Required package metadata

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Required package metadata

Every package entry MUST include:
- `package_id` (stable identifier)
- `version` (semver or monotonic)
- `publisher_id`
- `artifact_digest` (content hash)
- `signature` (publisher signature over digest)
- `manifest` (AGENT-MANIFEST-01 compliant)
- `capability_declarations` (requested tools + scopes)
- `sandbox_profile_id` (SANDBOX-01 mapping)
- `update_policy` (auto, prompt, or pinned)
- `provenance` (build or source attestations)
