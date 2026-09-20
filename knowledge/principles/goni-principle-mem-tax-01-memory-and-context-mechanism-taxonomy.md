---
id: GONI-PRINCIPLE-MEM-TAX-01
title: "Memory And Context Mechanism Taxonomy"
type: principle
status: draft
implementation_state: specified_only
proposition: "Raw history, compaction, summaries, retrieval, canonical state, episodic memory, and learned memory are distinct mechanisms with different information-loss, omission, staleness, and cost characteristics and should not be treated as interchangeable."
domains:
- memory
- system
aliases: []
relations:
- type: refines
  target: GONI-SYNTHESIS-D5A359AF7D66
- type: depends_on
  target: GONI-SPEC-C0EC9D0B0A06
sources:
- SRC-PACKER2023-MEMGPT
artifacts: []
uncertainty: "The preferred mixture of mechanisms remains an empirical architecture question and should be evaluated by workload."
legacy: []
---

# Memory And Context Mechanism Taxonomy

Goni should preserve explicit distinctions among raw interaction history, lossy compaction, generated summaries, query-driven retrieval, canonical structured state, episodic memory, and learned or weight-level memory.

These mechanisms fail differently. Raw history creates attention and cost pressure; summaries can delete nuance; retrieval can omit relevant state; structured state can lose unmodeled context; learned memory is difficult to inspect and reverse.

Memory architecture should be selected by the property the system needs to preserve rather than by context-window capacity alone.
