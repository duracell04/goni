---
id: GONI-EVIDENCE-MULTI-AGENT-01
title: Layered and dynamically composed multi-agent systems support graph-style cognition
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Mixture-of-Agents and AgentVerse provide empirical examples of combining multiple LLM agents through layered or dynamically composed collaboration, supporting evaluation of fan-out, specialist, and fan-in patterns inside a governed cognitive graph."
domains:
- research
- agent
- models
aliases: []
relations:
- type: supports
  target: COG-GRAPH-01
- type: supports
  target: MODEL-ROLE-01
sources:
- SRC-WANG2024-MIXTURE-OF-AGENTS
- SRC-CHEN2023-AGENTVERSE
artifacts: []
uncertainty: "Reported gains are benchmark- and architecture-specific. Multi-agent composition can increase cost, latency, correlated error, and integration burden; it is evidence for a candidate graph pattern rather than a default policy."
legacy: []
---

# Multi-agent work supports graph-style cognition

Mixture-of-Agents demonstrates layered composition in which multiple agents
produce candidate outputs and subsequent agents consume those outputs.
AgentVerse studies dynamically composed multi-agent groups across several task
settings.

These works provide evidence that fan-out, specialist roles, and fan-in
synthesis can sometimes improve task performance.

Goni adds constraints not established by those results:

- agent identity and model substrate remain separate;
- graph loops and fan-out remain budgeted;
- authority cannot emerge from agent consensus;
- specialist composition must be compared against simpler single-model and
  fixed-cascade baselines;
- verification and receipt provenance remain explicit.
