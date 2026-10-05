---
id: NET-LEARN-EVAL-01
title: Collective evidence adoption evaluation
type: experiment
status: draft
implementation_state: specified_only
proposition: A matched evaluation should test whether shared evidence produces useful
  candidate improvements while preserving privacy, evidence independence, regression
  boundaries, and owner-controlled adoption.
domains:
- learning
- evaluation
aliases: []
relations:
- type: tests
  target: NET-LEARN-01
- type: tests
  target: COOP-LEARNING-RISK-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Datasets, contribution threat models, export profiles, aggregation methods,
  and held-out acceptance thresholds require a pinned experiment implementation.
legacy: []
---

# Collective evidence adoption evaluation

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

Compare local-only improvement with cooperative evidence using identical
candidate-generation and total-resource budgets. Include shared provenance roots,
duplicate reports, contradictory scopes, fabricated/poisoned contributions,
selective success reporting, synthetic evidence, and privacy-sensitive exports.

Evaluate candidate patches on held-out tasks and owner-specific regression cases.
Measure decision utility, failure/regression rate, lineage handling, poisoning
sensitivity, disclosure, and unauthorized adoption. Include changed base revisions,
revoked adoption authority, rejected candidates, and rollback cases. Record
contribution permissions, anonymized/authorized lineage, aggregation policy,
target/base revision, full implementation/model/test revisions, and acceptance
receipts. Set improvement and regression thresholds before execution; require
zero unauthorized disclosures/adoptions in the named tested boundary. The plan
creates no empirical improvement or robustness claim.
