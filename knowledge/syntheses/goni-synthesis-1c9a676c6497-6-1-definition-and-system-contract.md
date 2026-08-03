---
id: GONI-SYNTHESIS-1C9A676C6497
title: 6.1 Definition and system contract
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Definition.** Proactive = policy-governed background cognition plus attention-aware interventions.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: 6.1 Definition and system contract
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# 6.1 Definition and system contract

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.1 Definition and system contract

**Definition.** Proactive = policy-governed background cognition plus
attention-aware interventions. Proactivity is a resource-constrained systems
problem (scheduling, admission control, interruption policy) and an OS
governance problem (non-bypassable boundaries for side effects and egress).

**Enforceable rules (normative).**
1) Scheduled: proactive work runs under a QoS class and budget (time, tokens,
   bandwidth, energy) before it can manifest as an interrupt. Queue growth and
   interactive tail latency must remain bounded. [R6, R7]
2) Justified: an intervention must satisfy an explicit expected-utility test
   relative to deferral and the expected cost of interruption. [R2-R5]
3) Accountable: any external side effect or egress is mediated through a
   reference-monitor boundary (capability syscall layer / net.egress) and
   leaves receipts with provenance semantics. [R9-R14]
