---
id: GONI-SPEC-FD6CC59ECC08
title: 2. Stages and roles
type: specification
status: draft
implementation_state: specified_only
proposition: 'The ITCR cascade is an asymmetric multi-stage pipeline: Stage 0 - Continuous state maintenance (low power) Encoders + predictor update latent state S_core and facts F_sparse.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/itcr.md
  heading: 2. Stages and roles
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 2. Stages and roles

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Stages and roles

The ITCR cascade is an asymmetric multi-stage pipeline:

Stage 0 - Continuous state maintenance (low power)
- Encoders + predictor update latent state S_core and facts F_sparse.
- Surprisal, risk, and goal conflict signals are computed.

Stage 1 - Proposal generation (cheap proposer)
- A small local model or heuristic planner emits a candidate plan and artifacts.
- Outputs are expected to be fast and fallible.

Stage 2 - Escalation policy (interrupt controller)
- The scheduler raises an ITCR interrupt only when expected value exceeds cost.
- Triggers are explicit predicates over state and request metadata.
- Hysteresis and rate limits prevent oscillation.

Stage 3 - ITCR reasoner/verifier (high power burst)
- The reasoner validates, scores, and repairs proposals.
- Search over candidates is bounded and budgeted.
- Verification is preferred over full regeneration.

Stage 4 - Commit under governance
- Execution only occurs after policy validation and schema checks.
- Tool calls are mediated by capability tokens and audit envelopes.
