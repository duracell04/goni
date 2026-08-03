---
id: GONI-SPEC-F9FED158CE1F
title: 3. Work Order binding
type: specification
status: draft
implementation_state: specified_only
proposition: Memory retrieval MUST be bound to work_order_id when retrieval is performed for delegated or tool-mediated work.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/memory-retrieval.md
  heading: 3. Work Order binding
  revision: 71a5e3aae5e865255619f7ba103fa2c7aa777941
---

# 3. Work Order binding

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Work Order binding

Memory retrieval MUST be bound to `work_order_id` when retrieval is performed
for delegated or tool-mediated work. Retrieval inputs SHOULD include:

- goal summary,
- done contract,
- task class,
- project/person constraints,
- policy hash,
- risk class,
- output shape.

If no Work Order exists, retrieval MUST either create one or record why a
read-only lookup was allowed without one.
