---
id: GONI-DECISION-1922DD722CB6
title: Consequences
type: decision
status: draft
implementation_state: specified_only
proposition: Hardware selection must provide the telemetry and knobs defined in blueprint/hardware/10-requirements.md.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/90-decisions.md
  heading: Consequences
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Consequences

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Consequences

- Hardware selection must provide the telemetry and knobs defined in
  `blueprint/hardware/10-requirements.md`.
- Scheduling policies in `blueprint/software/10-requirements.md` and
  `blueprint/30-specs/scheduler-and-interrupts.md` are mandatory for safe operation.
