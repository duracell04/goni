---
id: EVID-MCP-SEMANTICS-01
title: 'Source claim: MCP separates AI applications from tools and context resources'
type: evidence
status: draft
implementation_state: not_applicable
proposition: The Model Context Protocol defines interoperable application semantics for AI clients and servers, including tools, resources, and prompts, rather than being equivalent to a GUI, transport, or ordinary function-call syntax.
domains:
- research
- interoperability
- tools
aliases: []
relations:
- type: supports
  target: INTEROP-MCP-A2A-01
sources:
- SRC-MCP-2026-07-28
artifacts: []
uncertainty: MCP evolves through versioned specifications and extensions; Goni should pin any implementation claim to the protocol version actually supported.
legacy: []
---

# Source claim: MCP separates AI applications from tools and context resources

The current MCP specification defines server capabilities such as tools, resources, and prompts while treating transport and negotiated capabilities as separate protocol concerns.

For Goni, MCP is therefore best modeled as a capability and context interoperability boundary. It does not replace Goni's authority checks, sandboxing, egress policy, receipts, or execution verification.
