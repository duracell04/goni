---
id: GONI-DECISION-1A482B12FFB8
title: D-019 - Agents as local processes; solver as interrupt
type: decision
status: draft
implementation_state: specified_only
proposition: '**Status:** Proposed **Date:** YYYY-MM-DD **Formal statement** Agents are modeled as userland processes that operate on kernel-owned latent state and invoke capability-scoped syscalls.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/90-decisions.md
  heading: D-019 - Agents as local processes; solver as interrupt
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# D-019 - Agents as local processes; solver as interrupt

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## D-019 - Agents as local processes; solver as interrupt

**Status:** Proposed  
**Date:** YYYY-MM-DD

**Formal statement**

Agents are modeled as userland processes that operate on kernel-owned latent
state and invoke capability-scoped syscalls. LLM/solver execution is a
budgeted interrupt, not a control loop:
$$
\text{agent step} \Rightarrow \mathsf{read\_state} \to \mathsf{tool}^\* \to \mathsf{commit},
$$
and
$$
\mathsf{solver} \text{ is invoked iff } \text{interrupt\_condition} = \text{true}.
$$

**Rationale**

- Aligns with local-first power/thermal constraints.  
- Makes policy mediation and auditability explicit.  
- Prevents hidden LLM loops that burn budgets and drift state.

**Consequence**

- All agent effects must be routed through capability tokens and audit records.  
- Scheduler must enforce wake hysteresis and solver budgets.  
- Kernel APIs define the single interface for state access and commits.

*Amendment process:*  
New decisions should include:

- A short **formal statement** (equation, inequality, category-theoretic object, etc.).  
- A rationale explaining why the formal constraint is desirable.  
- Consequences for implementation and testing.

Changes to existing decisions are logged with a date and an explanation of why the previous constraint was no longer adequate.
