# Governance

GONI is currently maintained by its founder. The repository is designed to grow into academic-style review without pretending that review capacity already exists.

Canonical propositions change through reviewable topic branches and history-preserving merge commits. Every migrated proposition begins as draft. A later status promotion must identify the affected node IDs, address recorded objections, and cite the review decision. When two-reviewer governance becomes operational, status promotion and relation-vocabulary changes require two independent approvals; until then, such changes remain draft and the founder records the decision rationale.

Experimental repositories may submit pinned evidence or objections. They cannot automatically change node status, implementation state, or canonical wording.

Repository files are authoritative for operating rules:

- `schema/` defines machine contracts.
- `ontology/relations.yml` defines relations.
- `EDITORIAL_POLICY.md` defines knowledge and evidence boundaries.
- `COMMIT_STANDARD.md` defines historical provenance requirements.
- `CONTRIBUTING.md` defines the contributor workflow.
- `tooling/validate.py` is the executable strict validator.
