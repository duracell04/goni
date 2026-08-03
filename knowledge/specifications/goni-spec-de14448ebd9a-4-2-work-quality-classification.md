---
id: GONI-SPEC-DE14448EBD9A
title: 4.2 Work-quality classification
type: specification
status: draft
implementation_state: specified_only
proposition: 'The runtime MUST choose audit_grade when any of the following hold: the user asks for audit, verification, compliance, contribution review, status proof, or absence/presence of evidence, the task requires a negative claim such as "not present", "no contribution", "nothing changed", or "does not exist", the output can affect legal, financial, security, governance, publishing, or'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-interface.md
  heading: 4.2 Work-quality classification
  revision: e8be0d0ed13145f8f03d21a3aa00ca2e57a8fbe8
---

# 4.2 Work-quality classification

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.2 Work-quality classification

The runtime MUST choose `audit_grade` when any of the following hold:

- the user asks for audit, verification, compliance, contribution review,
  status proof, or absence/presence of evidence,
- the task requires a negative claim such as "not present", "no contribution",
  "nothing changed", or "does not exist",
- the output can affect legal, financial, security, governance, publishing, or
  irreversible operational decisions,
- the task depends on coverage across branches, repos, logs, PRs, issues,
  attachments, or build artifacts.

The runtime MAY choose `best_effort` only when the work is low-stakes,
reversible, and the user explicitly or implicitly accepts bounded exploration.
