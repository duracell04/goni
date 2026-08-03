---
id: GONI-PROPOSAL-90DE37204CD9
title: Change playbooks (required updates)
type: proposal
status: draft
implementation_state: specified_only
proposition: 'If you change schemas: update blueprint/software/50-data/51-schemas-mvp.md update blueprint/software/50-data/53-schema-dsl-and-macros.md update any relevant tests in goni-prototype-lab:software/kernel/goni-schema/tests/ If you change axioms/planes: update blueprint/software/50-data/10-axioms-and-planes.md If you change scheduling/escalation semantics: update blueprint/30-specs/scheduler-and-interrupts.md'
domains:
- agent
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/AGENTS.md
  heading: Change playbooks (required updates)
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Change playbooks (required updates)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Change playbooks (required updates)
- If you change schemas:
  - update `blueprint/software/50-data/51-schemas-mvp.md`
  - update `blueprint/software/50-data/53-schema-dsl-and-macros.md`
  - update any relevant tests in `goni-prototype-lab:software/kernel/goni-schema/tests/`
- If you change axioms/planes: update `blueprint/software/50-data/10-axioms-and-planes.md`
- If you change scheduling/escalation semantics: update `blueprint/30-specs/scheduler-and-interrupts.md`
- If you change tool execution/permissions: update `blueprint/30-specs/tool-capability-api.md`
- If you change hardware constraints: update `blueprint/hardware/10-requirements.md` and add/update ADR entries in `blueprint/hardware/90-decisions.md`

---
