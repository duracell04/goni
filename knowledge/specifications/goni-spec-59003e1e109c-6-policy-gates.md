---
id: GONI-SPEC-59003E1E109C
title: 6. Policy gates
type: specification
status: draft
implementation_state: specified_only
proposition: 'Before a bundle or governed model stack may process private memory, policy MUST check: license compatibility, source and publisher trust, hash match, assurance level, eval receipt coverage for the requested task class, adapter compatibility, adapter hashes, and adapter eval receipt coverage when adapters are active, prompt/policy bundle provenance and rollback state,'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/model-registry.md
  heading: 6. Policy gates
  revision: 8b0a4a359de641be0c57c4d8e40654f07d8bdda1
---

# 6. Policy gates

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Policy gates

Before a bundle or governed model stack may process private memory, policy MUST
check:

- license compatibility,
- source and publisher trust,
- hash match,
- assurance level,
- eval receipt coverage for the requested task class,
- adapter compatibility, adapter hashes, and adapter eval receipt coverage when
  adapters are active,
- prompt/policy bundle provenance and rollback state,
- memory or retrieval bundle refs and retention policy,
- private-memory permission,
- visual capability coverage and allowed asset class when the task is visual,
- visual workflow runtime provenance when a node graph or pipeline is used,
- network and retention policy for the runtime destination,
- policy pack provenance and override rules.

If any gate fails, the router MUST choose a safer approved bundle or block the
request.

Policy sources MUST be transparent, inspectable, and provenance-bearing. Goni
MUST support user-editable policies, signed policy packs, community or
enterprise overlays, and override receipts. Otherwise the local registry would
replace one hidden governance layer with another.
