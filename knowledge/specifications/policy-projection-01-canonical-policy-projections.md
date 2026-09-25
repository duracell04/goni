---
id: POLICY-PROJECTION-01
title: Canonical Policy Projections
type: specification
status: draft
implementation_state: specified_only
proposition: "Goni may compile canonical kernel policy into scoped derived eligibility projections for cognitive graphs, model routing, tools, network egress, MCP, and agent-to-agent communication, provided those projections cannot independently grant authority and remain bound to the canonical policy hash."
domains:
- kernel
- policy
- routing
- specs
aliases:
- POLICY-COMPILATION
relations:
- type: depends_on
  target: SPEC-POL-01
- type: depends_on
  target: COG-GRAPH-01
- type: depends_on
  target: ROUTE-SIGNAL-01
sources: []
artifacts: []
uncertainty: "This specification defines projection semantics, not a policy compiler implementation or a claim that every integration surface currently supports compiled enforcement."
legacy: []
---

# Canonical Policy Projections

## 1. One authority source

SPEC-POL-01 remains the normative source for capabilities, budgets,
information-flow constraints, validity windows, and revocation.

Goni may derive operational projections from that canonical policy for
different enforcement surfaces, but those projections are subordinate views.

Conceptually:

canonical policy
-> policy compiler
-> model eligibility
-> cognitive-graph eligibility
-> tool eligibility
-> network/egress eligibility
-> MCP eligibility
-> agent-to-agent eligibility.

A projection cannot create permission absent from the canonical policy.

## 2. Projection record

A derived policy projection SHOULD identify:

- canonical policy hash;
- projection type;
- projection version;
- compiler/version identity;
- Work Order or scope;
- creation time;
- validity or expiry;
- source policy refs;
- eligibility rules emitted;
- any conservative narrowing applied.

## 3. Monotonic authority boundary

Projection semantics SHOULD be authority-monotone:

the projection may preserve or reduce authority available under canonical policy
for its target surface, but it MUST NOT widen it.

For a canonical permission set P and projection P_i:

P_i is a subset of P

with respect to the actions and resources representable on that surface.

A projection may also add stricter operational constraints such as provider
allowlists, context minimization, or lower budgets.

## 4. Staleness and failure

If a projection cannot be proven to correspond to the active canonical policy,
or if its canonical policy hash is stale after revocation or policy update, the
affected surface SHOULD regenerate the projection or fail closed.

Cached routing or graph policy must not survive a relevant revocation merely
because cognition still has the old projection in context.

## 5. Separation from routing

The router consumes eligibility from policy projection together with cognitive
signals. It does not infer policy eligibility from semantic confidence.

Likewise, a cognitive graph edge may be disabled by policy projection, but a
graph transition cannot enable an action that policy denies.

## 6. Cross-layer consistency

Where the same canonical rule affects several surfaces, projection provides one
derivation point rather than multiple manually maintained policy copies.

Examples include:

- a private-data restriction that removes remote model routes and network
  egress simultaneously;
- a tool capability restriction that removes both a graph node and the
  corresponding tool syscall;
- a budget change that updates eligible model tiers and downstream tool spend
  limits;
- a revocation that invalidates agent-to-agent delegation and related graph
  edges together.

This reduces policy drift while preserving the kernel as the sole authority
boundary.
