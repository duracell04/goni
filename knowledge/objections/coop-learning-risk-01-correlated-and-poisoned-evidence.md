---
id: COOP-LEARNING-RISK-01
title: Aggregated evidence can amplify shared errors
type: objection
status: draft
implementation_state: specified_only
proposition: Collective learning can amplify correlated, selectively reported, or
  malicious contributions when their lineage, scope, and evaluation limits are unresolved.
domains:
- learning
- evaluation
aliases: []
relations:
- type: objects_to
  target: NET-LEARN-01
sources:
- SRC-GONI20261004-COOPERATIVE-COMPUTE
artifacts: []
uncertainty: Contribution independence, selection bias, and robustness under adversarial
  participation require an implemented aggregation policy and matched testing.
legacy: []
---

# Aggregated evidence can amplify shared errors

> Status boundary: this is a specified-only blueprint contract or planned evaluation.

Many contributions can descend from the same observation or model generation,
and a provider can submit persuasive records that omit failures. Privacy-limited
exports can make independence or representativeness difficult to establish.

NET-LEARN-01 addresses this objection through explicit unknown lineage,
contradictions, deduplication, poisoning assumptions and held-out candidate
evaluation. Local-only learning is a permitted alternative when external evidence
quality or confidentiality is insufficient. An aggregation rule's robustness
remains a measured property within a named threat model.
