---
id: GONI-PROPOSAL-ECECCCAD999E
title: What the full stack must provide
type: proposal
status: draft
implementation_state: specified_only
proposition: Owning only the model is insufficient.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/personal-ai-secretary-landscape.md
  heading: What the full stack must provide
  revision: 05bfea2b9178c594be35646dad31f9a0b6cab17e
---

# What the full stack must provide

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## What the full stack must provide

Owning only the model is insufficient. A durable personal operator needs five
separable layers:

1. **Inference:** a local runtime with swappable, hash-pinned open-weight model
   bundles, bounded context, resource reporting, and fully offline operation.
2. **Memory:** working, episodic, semantic, relational/project, and procedural
   records with provenance, retrieval, consolidation, export, and forgetting.
3. **Orchestration:** scheduled jobs, resumable work, interruption, retries,
   and explicit completion criteria.
4. **Tools:** scoped access to files, calendar, email, browser, code, and other
   systems, with credentials kept outside prompts.
5. **Authority:** a clear statement of which actor can approve effects, which
   policies bind execution, and how actions are reconstructed afterward.

Goni already specifies these concerns across its [LLM runtime](/blueprint/software/30-components/llm-runtime.md),
[governed memory retrieval](/blueprint/30-specs/memory-retrieval.md),
[scheduler](/blueprint/30-specs/scheduler-and-interrupts.md),
[tool capability API](/blueprint/30-specs/tool-capability-api.md), and
[receipts](/blueprint/30-specs/receipts.md). The missing work is integrated,
tested product behavior, not another personality prompt.
