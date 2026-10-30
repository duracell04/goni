---
id: GONI-SPEC-9E43C5648BB2
title: 3.6 Delegation and autonomy requirements (AUTON)
type: specification
status: draft
implementation_state: specified_only
proposition: 'Each task class has a declared corridor and explicit authority scope; autopilot and review behavior are bounded by current capabilities, enforceable policy preconditions, budgets, and effect-specific human approvals.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/10-requirements.md
  heading: 3.6 Delegation and autonomy requirements (AUTON)
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 3.6 Delegation and autonomy requirements

> Status boundary: the following is a specified contract, not a claim of deployed enforcement.

- **Autonomy corridors:** Each task class, including message drafting, invoice payment, calendar changes, and document editing, declares a `no_go`, `soft_gate`, or `autopilot` corridor together with allowed effect types and resource scopes.
  - `no_go` denies the specified effect.
  - `soft_gate` permits explicitly authorized low-risk effects marked for review after execution. Effects marked approval-required wait for approval before execution.
  - `autopilot` permits authorized effects inside the defined scope without routine approval, while escalating exceptions.
- **Risk-bounded operation:** Risk scores determine whether *already authorized* effects may proceed autonomously. A score never creates a capability or a mandate.
- **Approval versus review:** Human approval is a precondition for protected effects. An after-execution review is an audit mechanism and cannot retroactively authorize an effect.
- **Policy-level control:** Humans set corridors, capability scopes, thresholds, budgets, approval requirements, and revocation policies. The system minimizes routine interruptions within those bounds.
- **Offloading safeguards:** Provide anomaly-first review, periodic sampling, rapid corridor downgrades, and revocation controls.
- **Kernel-owned authority:** Capability tokens, policy decisions, budgets, receipts, and durable authority state remain kernel-owned.
- **Mediated extensions:** External assistant frameworks and newly generated workflows may contribute interfaces and execution capabilities through mediated tools; they do not own policy, canonical memory provenance, or session authority.

The operator interface can use `Restricted`, `Review`, and `Autopilot` as explanatory labels, while showing whether each individual action is denied, needs approval beforehand, or is eligible for post-execution review.
