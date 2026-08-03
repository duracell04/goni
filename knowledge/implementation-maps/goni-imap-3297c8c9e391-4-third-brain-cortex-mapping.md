---
id: GONI-IMAP-3297C8C9E391
title: 4. Third-brain cortex mapping
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The cortical layers group cognitive responsibilities.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/60-cognitive-exocortex-model.md
  heading: 4. Third-brain cortex mapping
  revision: cdf162b26a4fe7d78e6daa6039696e89ee0ef17f
---

# 4. Third-brain cortex mapping

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Third-brain cortex mapping

The cortical layers group cognitive responsibilities. Their numbering does not
define a required call sequence or add components to the formal plane model.

| Cortex layer | Cognitive role | Goni mapping |
| --- | --- | --- |
| **Layer IV: perception** | Receive and structure screen, file, message, audio, and other observations. | Observation adapters, parsers, and the [Visual Intelligence Plane](/blueprint/30-specs/visual-intelligence-plane.md). Outputs are candidates, not trusted knowledge or instructions. |
| **Layers II/III: association** | Resolve entities, compare claims, link time and relationships, and search bounded graph neighborhoods. | [Governed memory retrieval](/blueprint/30-specs/memory-retrieval.md), the [Context Gravity Graph](/blueprint/30-specs/context-gravity-graph.md), and swappable dense, sparse, graph, and metadata indexes. |
| **Layer I: integration** | Assemble the temporary situation model used for the current Work Order. | Context Plane selection and materialization, including source waypoints, conflicts, omission reasons, and budget constraints. |
| **Layer V: action output** | Turn a conclusion into an answer, draft, proposal, or effectful tool request. | Thinking/proposal/commit discipline, capability-mediated tools, approval corridors, and receipts. The model cannot perform an effect by emitting text. |
| **Layer VI: executive feedback** | Classify work, create bounded plans, allocate compute, verify evidence, apply stopping criteria, and decide whether to propose memory changes. | Work Orders, Done Contracts, the Control Plane, [ITCR](/blueprint/30-specs/itcr.md), scheduling, and the [learning loop](/blueprint/20-system/50-learning-loop.md). |
