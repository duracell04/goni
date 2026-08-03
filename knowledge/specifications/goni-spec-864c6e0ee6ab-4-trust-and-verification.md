---
id: GONI-SPEC-864C6E0EE6AB
title: 4. Trust and verification
type: specification
status: draft
implementation_state: specified_only
proposition: 'The runtime MUST: treat store indices as untrusted input.'
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
  heading: 4. Trust and verification
  revision: 17d060e8ac309d5a25765a07f3a00da85d0739e1
---

# 4. Trust and verification

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Trust and verification

The runtime MUST:
- treat store indices as untrusted input.
- verify signatures and digests before install or update.
- validate manifests against AGENT-MANIFEST-01.
- reject packages with undeclared capabilities or missing sandbox mappings.
- enforce store trust tiers for install and update policy.
