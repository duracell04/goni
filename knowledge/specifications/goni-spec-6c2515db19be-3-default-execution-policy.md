---
id: GONI-SPEC-6C2515DB19BE
title: 3. Default execution policy
type: specification
status: draft
implementation_state: specified_only
proposition: 'Goni executes only an explicitly authorized effect whose capability, scope, corridor, preconditions, budgets, risk constraints, and effect-specific approval requirements are all satisfied.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 3. Default execution policy
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 3. Default execution policy

> Status boundary: specified design only.

Goni uses policy-level, risk-bounded autonomy **after** authorization has been established.

1. Verify an active mandate, matching capability scope, current policy version, budget, expiry, revocation state, and all applicable resource and recipient constraints.
2. Apply the corridor and risk threshold as additional restrictions.
3. Apply the effect's review mode:
   - **Approval before execution:** Pause the effect until an authorized human approves the exact operation and its materially relevant inputs.
   - **Review after execution:** Execute only where the policy explicitly delegates that effect in advance; queue its receipt for later review.
4. Perform the effect through mediated execution and record its receipt.

Corridor defaults:

- `no_go`: Deny the specified effect.
- `soft_gate`: Low-risk, pre-authorized review-after effects may execute with queued review; actions requiring advance approval remain pending.
- `autopilot`: Eligible, pre-authorized effects execute within the declared risk threshold and budget; protected effects and exceptions escalate.

A missing, ambiguous, expired, or revoked permission leads to denial or an explicit authorization request. Risk classification alone cannot create authority. Thresholds, review modes, and authorization decisions are versioned and auditable.
