# Contributing

Start with `knowledge/README.md`, `EDITORIAL_POLICY.md`, and `ontology/relations.yml`. Propose one coherent concern per topic branch. Add or change atomic nodes; do not duplicate normative prose in indexes or syntheses.

Before review:

1. Run `python tooling/build_catalogue.py`.
2. Run `python tooling/validate.py --strict`.
3. Confirm a second catalogue build from the same tree is byte-identical.
4. Keep `maps/catalogue.json` and `maps/edition.json` untracked; they are generated projections, not reviewable source.
5. Use the message anatomy in `COMMIT_STANDARD.md`, listing every changed path.

New design nodes use `status: draft` and `implementation_state: specified_only`. Stronger implementation claims require full repository and commit references plus test evidence where applicable. New relations must use the controlled vocabulary. Planned checks belong under `knowledge/experiments/`, not `knowledge/evidence/`.
