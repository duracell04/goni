---
id: EVID-K8S-CONFIG-2026
title: "Source claim: Kubernetes configuration defect coverage"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Kubernetes configuration defects encompass categories not covered by all existing static analyzers."
domains:
- research
- delivery
- validation
aliases: []
relations:
- type: supports
  target: CI-ASSURANCE-01
sources:
- SRC-ZHANG2026-K8S-CONFIG
artifacts: []
uncertainty: "Kubernetes-specific study with 719 sampled defects and eight tested analyzers; no total-cost comparison of deployment architectures."
legacy: []
---

# Kubernetes configuration defect coverage — source claim

**Observation.** Zhang, Paul, d'Amorim, and Rahman classify 719 Kubernetes configuration defects into 15 categories. Eight studied static-analysis tools together cover eight of these categories.

**GONI relevance.** Syntax checks and general linters need targeted contract or runtime verification for consequential configuration behavior.

**Limit.** This study does not prove Kubernetes is always inferior to lighter deployment technologies.
