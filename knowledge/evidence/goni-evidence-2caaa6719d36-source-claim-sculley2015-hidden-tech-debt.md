---
id: GONI-EVIDENCE-2CAAA6719D36
title: 'Source claim: sculley2015-hidden-tech-debt'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Real ML systems contain far more than model code, and surrounding infrastructure can create hidden technical debt through glue code, configuration debt, undeclared consumers, hidden feedback loops, and boundary erosion.
domains:
- research
aliases: []
relations:
- type: supports
  target: DOCTRINE-DELEG-01
sources:
- SRC-SCULLEY2015-HIDDEN-TECH-DEBT
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[sculley2015-hidden-tech-debt]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: sculley2015-hidden-tech-debt

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[sculley2015-hidden-tech-debt]]
Claim: Real ML systems contain far more than model code, and surrounding
infrastructure can create hidden technical debt through glue code,
configuration debt, undeclared consumers, hidden feedback loops, and boundary
erosion.
Relevance:
- Supports treating Goni's non-model operating layer as first-class
  engineering surface rather than informal glue.
- Grounds Harness Governance as a way to keep prompts, policies, retrieval,
  routing, tools, receipts, and evals inspectable and reversible.
Used in:
- `blueprint/10-product/15-delegation-doctrine.md` (Harness Governance)
Source:
- https://papers.nips.cc/paper/5656-hidden-technical-debt-in-machine-learning-syst
