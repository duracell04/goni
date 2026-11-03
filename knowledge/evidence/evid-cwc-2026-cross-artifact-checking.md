---
id: EVID-CWC-2026
title: "Source claim: cross-artifact workflow checking"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Cross-analysis of workflow and application configuration can detect targeted CI/CD anti-patterns missed by syntax-only inspection."
domains:
- research
- delivery
- validation
aliases: []
relations:
- type: supports
  target: CI-ASSURANCE-01
sources:
- SRC-KIM2026-WORKFLOW-CHECKER
artifacts: []
uncertainty: "Four defined rule families in 1,984 Node.js/Python repositories; figures do not represent the prevalence of all CI/CD defects."
legacy: []
---

# Cross-artifact workflow checking — source claim

**Observation.** Kim and Lee's Continuous Workflow Checker evaluates GitHub Actions against application artifacts. It reports at least one targeted anti-pattern in 12.4% of sampled Node.js repositories and 7.0% of sampled Python repositories.

**GONI relevance.** Compare versions and deployment prerequisites with real application needs.

**Limit.** The preprint evaluates four anti-pattern families, not complete deployment correctness.
