---
id: GONI-EXPERIMENT-07A1AA51D44F
title: Pass criteria
type: experiment
status: draft
implementation_state: not_applicable
proposition: Direct egress attempt returns explicit deny/failure.
domains:
- validation
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/validation/EVID-ENF-01-egress-nonbypass.md
  heading: Pass criteria
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# Pass criteria

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Pass criteria

- Direct egress attempt returns explicit deny/failure.
- Missing-capability mediated request is denied with decision metadata.
- Valid-capability mediated request succeeds and is attributable.
- Each attempt path has exactly one terminal receipt entry.
