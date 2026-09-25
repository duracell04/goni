---
id: DELEG-AGENCY-01
title: Delegation as an agency-cost and observability problem
type: synthesis
status: draft
implementation_state: specified_only
proposition: Persistent AI delegation can be analyzed as a principal-delegate governance problem in which specification, monitoring, verification, residual error, interruption, and governance costs must be minimized without assuming that artificial delegates possess the incentive structure of human agents.
domains: [research, delegation, governance]
aliases:
- principal-agent lens
- delegation agency costs
relations:
- type: synthesizes
  target: EVID-AGENCY-COSTS-01
- type: synthesizes
  target: EVID-DELEGATION-OBSERVABILITY-01
- type: refines
  target: GONI-SYNTHESIS-FFFD0BDE80BE
sources:
- SRC-JENSEN1976-AGENCY-COSTS
- SRC-HOLMSTROM1979-OBSERVABILITY
artifacts: []
uncertainty: Classical principal-agent theory assumes incentive structures that need not apply to artificial delegates. Goni imports the delegation, observability, monitoring, and residual-loss lens rather than a claim of full economic equivalence.
legacy: []
---

# Delegation as an agency-cost and observability problem

Goni can be interpreted as infrastructure for reducing the total cost of useful delegated digital agency.

A provider-agnostic cost decomposition is:

[
C_D=C_{spec}+C_{monitor}+C_{verify}+C_{error}+C_{interrupt}+C_{govern}
]

where:

- (C_{spec}) is the cost of expressing objectives, constraints, and authority;
- (C_{monitor}) is the cost of observing delegated execution;
- (C_{verify}) is the cost of establishing whether outcomes satisfy the contract;
- (C_{error}) is expected residual loss from incorrect or unauthorized behavior;
- (C_{interrupt}) is the cost of unnecessary principal attention;
- (C_{govern}) is the operational cost of policy, identity, capability, receipt, and revocation infrastructure.

This reframes the product objective. Maximum autonomy is not the target. Minimum intervention is not the target. Maximum verification is not the target.

The target is **useful delegation at the lowest total governance cost compatible with principal sovereignty and consequence-sensitive assurance**.

Observability changes the optimum. When effects are reversible and postconditions are cheaply observable, the system can rationally allow wider execution corridors. When effects are opaque, irreversible, high-consequence, or difficult to verify, the same cognitive capability may justify a narrower authority corridor.
