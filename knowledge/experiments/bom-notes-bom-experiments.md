---
id: BOM-NOTES
title: BOM Experiments
type: experiment
status: draft
implementation_state: not_applicable
proposition: This folder contains BOM snapshots and notes used to sanity-check technical feasibility.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/50-bom-experiments/bom-notes.md
  heading: BOM Experiments
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# BOM Experiments

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

# BOM Experiments

This folder contains BOM snapshots and notes used to sanity-check technical feasibility.

Guidelines:

- Do not overwrite old BOMs: create a new file (v2, v3, ...) with a snapshot date.
- Anchor critical parts to public vendor specifications (memory class, IO, thermals, power envelope).
- Keep component alternatives as ranges until mechanical and thermal tests are complete.
- Keep BOMs consistent with current decisions in [`../90-decisions.md`](/blueprint/hardware/90-decisions.md).

Current files:

- `bom-v1-apu-node.md` - early technical draft.
- `bom-v2-framework-395-128gb.md` - current MVP technical snapshot.
