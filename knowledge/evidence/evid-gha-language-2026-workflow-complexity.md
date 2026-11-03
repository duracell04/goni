---
id: EVID-GHA-LANGUAGE-2026
title: "Source claim: GitHub Actions language complexity"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "More complex GitHub Actions workflows are associated with elevated failure and maintenance burden in an observational sample."
domains:
- research
- delivery
aliases: []
relations:
- type: supports
  target: DELIVERY-MIN-01
sources:
- SRC-BARDSIRI2026-GHA-LANGUAGE
artifacts: []
uncertainty: "Observational association; neither a causal complexity threshold nor GONI-specific effects were established."
legacy: []
---

# GitHub Actions language complexity — source claim

**Observation.** Talebzadeh Bardsiri, Decan, and Mens (ICSME 2026) analyze roughly 260,000 workflows across 49,000 repositories and identify 197 language constructs in 14 features. Larger and more complex workflows were associated with more failures and maintenance work.

**GONI relevance.** Include the lifecycle burden of delivery mechanisms in architecture decisions.

**Limit.** No causal conclusion that removing a specific workflow step improves reliability.
