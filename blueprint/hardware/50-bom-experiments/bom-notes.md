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
