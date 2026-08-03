---
id: GONI-SPEC-2DF2DAAC5797
title: 6. Receipts and auditability
type: specification
status: draft
implementation_state: specified_only
proposition: 'Install, update, and removal MUST emit receipts including: package_id, version, artifact_digest publisher_id, signature_id capability policy snapshot hash sandbox_profile_id trust tier at time of action'
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
  heading: 6. Receipts and auditability
  revision: 17d060e8ac309d5a25765a07f3a00da85d0739e1
---

# 6. Receipts and auditability

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Receipts and auditability

Install, update, and removal MUST emit receipts including:
- package_id, version, artifact_digest
- publisher_id, signature_id
- capability policy snapshot hash
- sandbox_profile_id
- trust tier at time of action
