---
id: GONI-SPEC-8BFAE04A83C6
title: 2. Core Capabilities
type: specification
status: draft
implementation_state: specified_only
proposition: 'At a minimum, a single Goni node should: Provide an **interactive conversational assistant** with: natural language chat, optional voice input and output (when peripherals are available), memory of context within a session.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 2. Core Capabilities
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 2. Core Capabilities

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Core Capabilities

At a minimum, a single Goni node should:

1. Provide an **interactive conversational assistant** with:
   - natural language chat,
   - optional voice input and output (when peripherals are available),
   - memory of context within a session.

2. Support **retrieval-augmented generation (RAG)** over:
   - local documents and notes,
   - emails and calendar entries (if connected),
   - other user-approved data sources.

3. Offer a **coding assistant** behaviour:
   - explanation of code,
   - basic code generation,
   - summarisation of diffs / pull requests,
   - without requiring external cloud access for most tasks.

4. Perform **lightweight model personalisation**:
   - training small adapters or similar techniques on user data,
   - within the compute limits of the device,
   - without full retraining of large base models.

5. Expose a **network API** suitable for:
   - integration with local tools (editors, terminals, browsers),
   - remote access from the user’s other devices (laptop, phone, tablet).

---
