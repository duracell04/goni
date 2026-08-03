---
id: GONI-EVIDENCE-25F334DB139D
title: 'Source claim: frugalgpt2023'
type: evidence
status: draft
implementation_state: not_applicable
proposition: LLM cascades, prompt adaptation, and LLM approximation can reduce inference cost while preserving or improving task quality.
domains:
- research
aliases: []
relations:
- type: supports
  target: DOCTRINE-DELEG-01
- type: supports
  target: GONI-IMAP-45DA8323C140
sources:
- SRC-FRUGALGPT2023
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[frugalgpt2023]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: frugalgpt2023

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[frugalgpt2023]]
Claim: LLM cascades, prompt adaptation, and LLM approximation can reduce
inference cost while preserving or improving task quality.
Relevance:
- Grounds Goni's cascade pattern: cheap/private/local routes should be tried
  before expensive external routes.
- Goni extends the objective from cost-quality to sovereignty, privacy,
  latency, energy, auditability, and policy compatibility.
Used in:
- `blueprint/10-product/15-delegation-doctrine.md` (Frugal Sovereign Routing)
- `blueprint/software/20-architecture.md` (Frugal sovereign model router)
Source:
- https://huggingface.co/papers/2305.05176
