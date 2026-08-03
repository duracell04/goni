---
id: GONI-SYNTHESIS-4AF79FF106F0
title: Spec vs implementation discipline
type: synthesis
status: draft
implementation_state: specified_only
proposition: MUST treat normative specs as canonical contracts.
domains:
- agent
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/meta/agents.root.template.md
  heading: Spec vs implementation discipline
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Spec vs implementation discipline

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Spec vs implementation discipline
- MUST treat normative specs as canonical contracts.
- MUST update specs/ADRs when changing invariants, interfaces, schemas, or hardware assumptions.
- MUST NOT create competing sources of truth by duplicating definitions across docs.
