---
id: EVID-AGENT-OPS-2026
title: "Source claim: agent deployment operational contracts"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "Container lifecycle events can violate an agent application's state-persistence and resumability assumptions."
domains:
- research
- delivery
- kernel
aliases: []
relations:
- type: supports
  target: OPS-CONTRACT-01
sources:
- SRC-MASTOURI2026-AGENT-OPS
artifacts: []
uncertainty: "October 2026 preprint derived from particular frameworks and sampled applications; applicability to GONI requires testing."
legacy: []
---

# Agent deployment operational contracts — source claim

**Observation.** Mastouri, Ksontini, and Barrak derive 14 operational contracts and examine 433 deployed agent systems, lifecycle faults in 12 systems, and developer repairs. They identify gaps between disposable containers and applications expecting persistent, resumable state.

**GONI relevance.** Make durable action intent, receipt state, approvals, and recovery part of the application-deployment contract.

**Limit.** The study does not establish that a particular orchestration platform or database is optimal.
