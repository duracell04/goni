---
id: GONI-IMAP-75C7500299DD
title: 4. Mapping API calls to planes
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Serving a single HTTP request proceeds conceptually as: **Orchestrator** Normalises \(R\) into a JobDescriptor: J = (\text{class}, \text{budget}, \text{tools}, \text{profile}, \text{interaction\_mode}, \text{work\_order\_ref}, \dots) where: class ∈ {interactive, background, maintenance}, budget encodes token/time limits,'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 4. Mapping API calls to planes
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 4. Mapping API calls to planes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Mapping API calls to planes

Serving a single HTTP request proceeds conceptually as:

1. **Orchestrator**
   Normalises \(R\) into a JobDescriptor:
   
   J = (\text{class}, \text{budget}, \text{tools}, \text{profile},
   \text{interaction\_mode}, \text{work\_order\_ref}, \dots)
   
   where:

   * class ∈ {interactive, background, maintenance},
   * budget encodes token/time limits,
   * interaction_mode ∈ {delegation, co_creation}.

   Request normalization also compiles a Work Order and Done Contract reference
   before corridor policy or tool mediation.

2. **Control Plane (\mathcal{K})**

   * Enqueues \(J\) into the appropriate class queue.
   * Scheduler (MaxWeight) decides when it runs.
   * Router decides which model tier to use (goni-small / goni-large / etc).

3. **Context Plane (\mathcal{X})** (if RAG enabled)

   * Retrieves candidate chunks from VecDB.
   * Runs submodular selection under token budget to choose context set \(S \subseteq V\).
   * Builds a PromptPlan.

4. **Execution Plane (\mathcal{E})**

   * LLM Runtime runs inference on the PromptPlan with chosen model.
   * Emits a token stream.

5. **Data Plane (\mathcal{A})**

   * Records metrics, traces, and tool outputs as Arrow tables.

These steps implement the abstract \(\mathsf{Serve}\) function.

> **Invariant API-3 (kernel-backed reconstruction)**
> Any `goni_reconstruction` object returned by the API must be derivable from
> Work Order state and policy state. Clients may render it differently, but
> they may not invent or mutate its substance.

---
