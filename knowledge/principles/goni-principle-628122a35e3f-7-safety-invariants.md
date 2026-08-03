---
id: GONI-PRINCIPLE-628122A35E3F
title: 7. Safety invariants
type: principle
status: draft
implementation_state: specified_only
proposition: CDC MUST NOT silently personalize system behavior.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: 7. Safety invariants
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# 7. Safety invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Safety invariants

- CDC MUST NOT silently personalize system behavior.
- Raw user or draft text MUST NOT be stored in receipts by default.
- Candidate rules MUST preserve provenance and source refs sufficient for audit.
- Contradictions MUST narrow scope, reduce confidence, or require review.
- High-risk, privacy, legal, financial, or constitutional preferences MUST
  require explicit approval before promotion.
- Retrieval, prompt, routing, or tool-policy changes MUST attach only to
  declared seams in the Learning Loop.
- Untrusted source text MUST NOT become control-plane instruction without policy
  mediation.
