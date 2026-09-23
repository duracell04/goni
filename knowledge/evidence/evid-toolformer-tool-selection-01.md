---
id: EVID-TOOLFORMER-TOOL-SELECTION-01
title: 'Source claim: Toolformer studies model-selected API use'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Toolformer studies language models learning when to call external APIs, which API to use, what arguments to pass, and how to incorporate returned results.
domains:
- research
- tools
aliases: []
relations:
- type: supports
  target: AGENT-ONTOLOGY-01
sources:
- SRC-SCHICK2023-TOOLFORMER
artifacts: []
uncertainty: Toolformer addresses learned tool-use behavior rather than authorization or enforceable execution policy.
legacy: []
---

# Source claim: Toolformer studies model-selected API use

Schick et al. show that a language model can learn decisions about when and how to call external APIs.

For Goni, this is evidence for separating a model's proposed tool choice from the runtime that validates, authorizes, executes, and records the resulting effect.
