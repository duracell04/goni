---
id: GONI-EVIDENCE-074976FCCFF9
title: 'Source claim: greshake2023-indirect-prompt-injection'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Untrusted retrieved text can inject instructions that hijack tool use and control flow in LLM-integrated systems.
domains:
- research
aliases: []
relations:
- type: supports
  target: PRIV-01
sources:
- SRC-GRESHAKE2023-INDIRECT-PROMPT-INJECTION
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[greshake2023-indirect-prompt-injection]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: greshake2023-indirect-prompt-injection

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[greshake2023-indirect-prompt-injection]]
Claim: Untrusted retrieved text can inject instructions that hijack tool use
and control flow in LLM-integrated systems.
Relevance:
- Motivates separating untrusted text from control/execution planes.
- Supports redaction and minimization before remote escalation.
Used in:
- `blueprint/software/50-data/40-privacy-and-text-confinement.md` (Empirical motivation)
