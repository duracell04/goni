---
id: GONI-PRINCIPLE-9726D0E382F2
title: 10. Safety Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: Graph edges MUST NOT turn untrusted source text into Control Plane instruction without policy mediation.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/context-gravity-graph.md
  heading: 10. Safety Invariants
  revision: ab7b91df1b7045160319da054907e6304e6dcc76
---

# 10. Safety Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 10. Safety Invariants

- Graph edges MUST NOT turn untrusted source text into Control Plane
  instruction without policy mediation.
- Graph traversal MUST NOT increase the authority of observed screen, browser,
  OCR, audio, or accessibility-derived material beyond its memory grant.
- Private or relationship-scoped edges MUST NOT cause remote context disclosure
  unless policy explicitly allows the destination and purpose.
- Deletion, redaction, tombstoning, or permission revocation MUST remove or
  demote affected nodes and edges from normal traversal.
- Edge extraction confidence MUST remain visible to scoring and receipts.
- Graph-derived uncertainty SHOULD be surfaced when conflicting edges materially
  affect selected context.
- User-specified weights MUST remain inspectable and MUST NOT be silently
  overridden by inferred or reinforced weights.
