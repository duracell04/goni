---
id: EVID-INSTRUCTION-HIERARCHY-01
title: 'Source claim: instruction privilege matters for prompt-injection robustness'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Wallace et al. identify the absence of instruction privilege as a prompt-injection vulnerability and demonstrate a training approach that prioritizes higher-privilege instructions over conflicting lower-privilege text.
domains:
- research
- safety
- context
aliases: []
relations:
- type: supports
  target: TRUST-INPUT-01
sources:
- SRC-WALLACE2024-INSTRUCTION-HIERARCHY
artifacts: []
uncertainty: Model-level instruction hierarchy improves robustness but does not substitute for deterministic Goni authority and information-flow boundaries.
legacy: []
---

# Source claim: instruction privilege matters for prompt-injection robustness

The Instruction Hierarchy work treats conflicting text according to privilege rather than as undifferentiated prompt content.

For Goni, this is evidence for preserving trusted policy, user-authorized intent, tool observations, retrieved documents, and arbitrary external text as different trust classes.
