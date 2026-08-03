---
id: GONI-SYNTHESIS-E2C3130BEB6E
title: 2.2 Patch seams and allowed attachment points
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Candidate changes may attach only to declared seams: S1 router seam: routing rules, thresholds, and expert selection.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/50-learning-loop.md
  heading: 2.2 Patch seams and allowed attachment points
  revision: facf4ec813a02ec315fbe482a25bdac18686846e
---

# 2.2 Patch seams and allowed attachment points

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2.2 Patch seams and allowed attachment points
Candidate changes may attach only to declared seams:

- S1 router seam: routing rules, thresholds, and expert selection.
- S2 expert seam: per-expert adapters, sparse deltas, or small modules.
- S3 trunk-interface seam: stable output contracts, refusal policy, and schema.
- S4 retrieval seam: indexes, reranker config, citation rules, memory shaping.
- S5 tool-policy seam: capabilities, corridors, and two-phase write policy.

Delegation-policy bundles attach at S5. This includes clarification thresholds,
assumption-visibility rules, corridor defaults, and irreversible-action policy.
They are treated as reversible control-plane patches rather than hidden prompt
edits [[tomasev2026-intelligent-delegation]] [[zhang2025-ace]].

Undeclared attachment points are rejected. A patch that cannot name its seam is
not a valid patch.
