---
id: EVID-SAFE-TOOL-USE-01
title: 'Source claim: safe tool use requires enforceable constraints beyond model safeguards'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Doshi et al. argue that model-based safeguards cannot by themselves guarantee safe agent tool use and propose formalized, enforceable constraints on data flows and tool sequences.
domains:
- research
- safety
- tools
aliases: []
relations:
- type: supports
  target: TRUST-INPUT-01
sources:
- SRC-DOSHI2026-SAFE-TOOL-USE
artifacts: []
uncertainty: The paper proposes a safety-engineering process and capability-enhanced MCP framework; Goni should not claim equivalent guarantees without its own implemented and verified enforcement.
legacy: []
---

# Source claim: safe tool use requires enforceable constraints beyond model safeguards

Doshi et al. distinguish reliability improvements from enforceable safety guarantees and formalize requirements over information flows and tool sequences.

This supports Goni's separation between a model's proposed action and kernel-owned execution authority.
