---
id: GONI-PRINCIPLE-90FEE084B108
title: 6.7 Security principles and reference monitor lineage
type: principle
status: draft
implementation_state: specified_only
proposition: 'Saltzer & Schroeder provide the canonical secure design checklist: least privilege, complete mediation, economy of mechanism, fail-safe defaults, separation of privilege, psychological acceptability.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: 6.7 Security principles and reference monitor lineage
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# 6.7 Security principles and reference monitor lineage

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.7 Security principles and reference monitor lineage

Saltzer & Schroeder provide the canonical secure design checklist: least
privilege, complete mediation, economy of mechanism, fail-safe defaults,
separation of privilege, psychological acceptability. [R11]

Reference monitor lineage traces to the Anderson planning study. [R12]

Goni mapping (normative):
- Least privilege: minimal capabilities per tool/agent.
- Complete mediation: every access and side effect is checked.
- Economy of mechanism: keep the trusted kernel small.
- Psychological acceptability: permissions must be understandable.
