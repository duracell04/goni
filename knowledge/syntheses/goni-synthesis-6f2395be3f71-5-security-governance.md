---
id: GONI-SYNTHESIS-6F2395BE3F71
title: 5) Security / governance
type: synthesis
status: draft
implementation_state: specified_only
proposition: egress decision latency bytes egressed per request + per domain blocked egress attempts and redaction events receipt coverage (% of tool calls + egress with receipts) receipt verification time + log growth rate sandbox escape attempts detected policy bypass attempts approval accuracy (false-approve and false-deny rates)
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/metrics/nonbypass-integration-metrics.md
  heading: 5) Security / governance
  revision: b3b8ab0b1b62416851f3d95b02d0aa711d322d6d
---

# 5) Security / governance

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5) Security / governance

- egress decision latency
- bytes egressed per request + per domain
- blocked egress attempts and redaction events
- receipt coverage (% of tool calls + egress with receipts)
- receipt verification time + log growth rate
- sandbox escape attempts detected
- policy bypass attempts
- approval accuracy (false-approve and false-deny rates)
- poisoning acceptance rate (untrusted content promoted to durable memory)
- trace replayability pass rate (equivalent inputs/policy reproduce decisions)
- TCB size (even approximate early on)
