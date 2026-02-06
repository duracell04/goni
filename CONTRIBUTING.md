# CONTRIBUTING

This repository is blueprint-only. Runnable artifacts live in `goni-prototype-lab`.

Quick start (implementation):
- See `goni-prototype-lab:goni-lab/quickstart.md` for setup and demo steps.
- Run tests and smoke checks from `goni-prototype-lab`, not from this repo.

Spec changes:
- If you change `blueprint/docs/specs`, add an RFC in `blueprint/docs/rfcs/` and update `goni-prototype-lab:goni-lab/TRACEABILITY.md`.
- If you change schemas, update `blueprint/software/50-data/51-schemas-mvp.md` and `blueprint/software/50-data/53-schema-dsl-and-macros.md` (see `blueprint/AGENTS.md`).

Evidence:
- Claims of enforcement require tests or benchmarks in `goni-prototype-lab` with a direct reference.
