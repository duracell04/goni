---
id: GONI-PRINCIPLE-8FC6EE7AB8FF
title: 4. Invariants (tied to 30-conformance)
type: principle
status: draft
implementation_state: specified_only
proposition: '**Admission invariant (K1)** The Orchestrator must apply request-level validation so that the global utilisation target in ??'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/orchestrator.md
  heading: 4. Invariants (tied to 30-conformance)
  revision: 6679267b9add139fa50e9ad7abf0642b9a2943cf
---

# 4. Invariants (tied to 30-conformance)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Invariants (tied to 30-conformance)

* **Admission invariant (K1)**
  The Orchestrator must apply request-level validation so that the global utilisation target in ?? is attainable, e.g. rejecting any single request whose budget would obviously violate
  
  \sum_i \lambda_i/\mu_i^{\max} < \alpha.
  

* **Plane separation invariant**
  All long-running work must enter through the Control Plane; the Orchestrator must *not* spin its own worker pools that bypass ??.

* **Local-first invariant**
  In offline mode, the Orchestrator may not introduce network dependencies; it just routes between local ??, ??, ??, ??.

---
