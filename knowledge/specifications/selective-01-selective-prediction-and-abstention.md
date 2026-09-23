---
id: SELECTIVE-01
title: Selective Prediction and Abstention Contract
type: specification
status: draft
implementation_state: specified_only
proposition: "Confidence-bearing cognition should support an explicit accept-versus-abstain selection policy whose quality is evaluated by coverage-conditioned risk rather than by unconditional accuracy alone."
domains:
- evaluation
- models
- routing
- specs
aliases:
- RISK-COVERAGE-CONTRACT
relations:
- type: depends_on
  target: CAL-01
- type: refines
  target: GONI-IMAP-EB2133E6965D
- type: refines
  target: GONI-SPEC-8911D8AA7EE0
sources:
- SRC-GEIFMAN2019-SELECTIVENET
artifacts: []
uncertainty: "Threshold form is a minimal abstraction. Real selectors may use richer features, conformal methods, ensembles, verification signals, or task-specific policies."
legacy: []
---

# Selective Prediction and Abstention Contract

Let f(x) produce a candidate prediction and let g(x) in {0,1} determine whether
the system accepts that prediction.

For a simple confidence threshold:

g_tau(x) = 1[p_hat(x) >= tau].

Coverage is the accepted fraction of the operating distribution:

phi(tau) = E[g_tau(X)].

Given task loss l, selective risk is:

R_sel(tau) = E[l(f(X),Y) g_tau(X)] / E[g_tau(X)],

when coverage is non-zero.

The system SHOULD evaluate a family of operating points rather than reporting
only one unconditional accuracy number. Risk-coverage curves make explicit how
reliability changes as autonomous coverage expands or contracts.

Abstention is a valid cognitive outcome. It may route to:

- a stronger local model;
- a deterministic checker or tool;
- a multi-model verifier;
- a remote model where policy permits;
- principal review where the unresolved uncertainty is principal-owned.

Abstention does not imply human interruption by default.

Selection is separate from authority. Acceptance by a selector means only that
the cognitive result passed the configured epistemic operating point. Any
effectful action remains subject to kernel policy and canonical authority state.
