---
id: EVID-AGENTDOJO-UNTRUSTED-DATA-01
title: 'Source claim: untrusted tool data can hijack agent behavior'
type: evidence
status: draft
implementation_state: not_applicable
proposition: AgentDojo evaluates tool-using agents in environments where untrusted data can contain prompt injections that attempt to redirect the agent toward malicious tasks.
domains:
- research
- safety
- tools
aliases: []
relations:
- type: supports
  target: TRUST-INPUT-01
sources:
- SRC-DEBENEDETTI2024-AGENTDOJO
artifacts: []
uncertainty: AgentDojo is an evaluation environment; successful defenses in that benchmark do not establish Goni-wide security guarantees.
legacy: []
---

# Source claim: untrusted tool data can hijack agent behavior

AgentDojo operationalizes the distinction between instructions originating from the principal and adversarial content arriving through tools or data sources.

Goni should therefore retain provenance and trust labels across retrieval and tool boundaries rather than flattening all text into one undifferentiated context stream.
