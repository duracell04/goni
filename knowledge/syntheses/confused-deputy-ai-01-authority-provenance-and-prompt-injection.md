---
id: CONFUSED-DEPUTY-AI-01
title: Authority provenance and the confused-deputy pattern
type: synthesis
status: draft
implementation_state: specified_only
proposition: Prompt injection and untrusted tool or document content can be analyzed partly as authority-confusion failures: data may influence cognition but must not acquire the authority of the principal, policy layer, or capability issuer merely by entering model context.
domains: [security, authority, context, delegation]
aliases:
- semantic confused deputy
- authority confusion
relations:
- type: synthesizes
  target: EVID-CONFUSED-DEPUTY-01
- type: refines
  target: TRUST-INPUT-01
- type: depends_on
  target: DELEG-AUTHORITY-01
sources:
- SRC-HARDY1988-CONFUSED-DEPUTY
- SRC-DEBENEDETTI2024-AGENTDOJO
- SRC-DOSHI2026-SAFE-TOOL-USE
artifacts: []
uncertainty: Confused-deputy analysis explains one important authority-confusion structure but does not subsume every prompt-injection, data-poisoning, or adversarial-content failure.
legacy: []
---

# Authority provenance and the confused-deputy pattern

A delegated agent can legitimately possess authority from a principal while simultaneously reading instructions, requests, documents, web pages, tool results, or messages produced by other actors.

The security question is therefore not only whether content is malicious. It is:

> **Under whose authority would the requested effect occur?**

Goni should preserve two separate provenance channels:

[
information provenance 
eq authority provenance
]

External content can change beliefs about the world. It cannot by itself promote its author into an authority source or widen the active delegation.

This connects Goni's prompt-injection posture to classical capability security: the kernel authorizes effects according to the valid delegation and current policy state, while model context remains an information environment whose contents carry explicitly classified trust.
