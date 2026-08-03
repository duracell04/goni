---
id: GONI-OBJECTION-668817DB7BE0
title: Failure modes and fallbacks
type: objection
status: draft
implementation_state: not_applicable
proposition: 'If telemetry is missing or unstable, Goni MUST fall back to conservative policies: reduce duty cycle, prefer CPU/iGPU routing, and defer compaction until safe conditions return.'
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/90-decisions.md
  heading: Failure modes and fallbacks
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Failure modes and fallbacks

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Failure modes and fallbacks

If telemetry is missing or unstable, Goni MUST fall back to conservative
policies: reduce duty cycle, prefer CPU/iGPU routing, and defer compaction until
safe conditions return.
