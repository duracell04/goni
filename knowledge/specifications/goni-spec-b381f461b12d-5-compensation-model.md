---
id: GONI-SPEC-B381F461B12D
title: 5. Compensation model
type: specification
status: draft
implementation_state: specified_only
proposition: 'Compensation applies to non-reversible operations: define compensation action (undo, counter-action, operator-task), attach compensation status in receipt chain, require explicit policy for auto-compensation vs manual escalation.'
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md
  heading: 5. Compensation model
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 5. Compensation model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Compensation model

Compensation applies to non-reversible operations:
- define compensation action (`undo`, `counter-action`, `operator-task`),
- attach compensation status in receipt chain,
- require explicit policy for auto-compensation vs manual escalation.

Compensation is mandatory for classes:
- outbound communication (`email.send`, `webhook.post`),
- financial/external state mutation,
- third-party configuration changes.
