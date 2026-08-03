---
id: GONI-DECISION-B252EB479E2B
title: D-013 – Metrics as first-class objects in \(\mathcal{A}\)
type: decision
status: draft
implementation_state: specified_only
proposition: '**Formal statement** All metrics, logs and traces are represented as objects in \(\mathcal{A}\): There exists a schema \(S_\text{log}\) for logs, \(S_\text{metric}\) for metrics, etc.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-013 – Metrics as first-class objects in \(\mathcal{A}\)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-013 – Metrics as first-class objects in \(\mathcal{A}\)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-013 – Metrics as first-class objects in \(\mathcal{A}\)

**Formal statement**

All metrics, logs and traces are represented as objects in \(\mathcal{A}\):

- There exists a schema \(S_\text{log}\) for logs, \(S_\text{metric}\) for metrics, etc.  
- Emission of metrics is a morphism:
  $$
  \mathsf{Emit} : S \to S \oplus S_\text{metric}
  $$
  in \(\mathcal{A}^{\text{cold}}\).

**Rationale**

- Makes metrics queryable using the same columnar tooling as user data.  
- Enables offline, local analysis of behaviour without relying on external telemetry.

**Consequence**

- We must define stable schemas for these “meta” objects and version them.  
- External observability systems (Prometheus, OTLP, etc.) are treated as sinks fed from \(\mathcal{A}\), not as authoritative stores.

---
