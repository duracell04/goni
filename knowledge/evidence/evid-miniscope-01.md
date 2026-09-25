---
id: EVID-MINISCOPE-01
title: 'Source claim: least-privilege agent authorization can be derived from tool-call permission structure'
type: evidence
status: draft
implementation_state: not_applicable
proposition: MiniScope reconstructs permission hierarchies over tool calls and uses them to confine tool-calling agents under a least-privilege authorization model.
domains: [research, security, tools]
aliases: []
relations:
- type: supports
  target: GONI-SYNTHESIS-DF627E719B9D
sources: [SRC-ZHU2025-MINISCOPE]
artifacts: []
uncertainty: MiniScope's properties apply to its evaluated setup; Goni should import the least-privilege design lesson rather than its implementation assumptions.
legacy: []
---

# Least privilege can be represented independently of model reasoning

MiniScope reinforces the provider-agnostic principle that authority can be externally represented and mechanically enforced around tool use.
