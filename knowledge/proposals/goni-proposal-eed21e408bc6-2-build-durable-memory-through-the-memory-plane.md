---
id: GONI-PROPOSAL-EED21E408BC6
title: 2. Build durable memory through the Memory Plane
type: proposal
status: draft
implementation_state: specified_only
proposition: Preserve transient working context separately from episodic events, semantic facts, project/relational knowledge, and versioned procedures.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: 2. Build durable memory through the Memory Plane
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# 2. Build durable memory through the Memory Plane

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2. Build durable memory through the Memory Plane

- Preserve transient working context separately from episodic events,
  semantic facts, project/relational knowledge, and versioned procedures.
- Store source, timestamp, permissions, confidence, validity, and expiry with
  every durable entry.
- Provide exact/sparse, semantic, metadata, and relationship retrieval, with
  reranking and low-confidence rejection.
- Make durable records inspectable and exportable; keep any vector or graph
  index rebuildable from canonical records.
- Implement consolidation, correction, pinning, expiry, forgetting, and audit
  as explicit jobs rather than model-side intuition.
