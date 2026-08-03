---
id: GONI-EXPERIMENT-6B6884CBF1FC
title: Test matrix
type: experiment
status: draft
implementation_state: not_applicable
proposition: 'Profile A: strict deny-all egress for tool runners.'
domains:
- validation
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/validation/EVID-ENF-01-egress-nonbypass.md
  heading: Test matrix
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# Test matrix

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Test matrix

- Profile A: strict deny-all egress for tool runners.
- Profile B: mediated allowlist egress via gateway.
- Profile C: policy-engine unavailable (fail-closed behavior).
