---
id: ROUTE-SIGNAL-01
title: Typed Cognitive Routing Signals
type: specification
status: draft
implementation_state: specified_only
proposition: "Cognitive routing should consume a versioned typed signal set whose fields identify task, difficulty, confidence, privacy, risk, freshness, context, cost, latency, energy, tool, and policy conditions without collapsing their distinct semantics into one opaque score."
domains:
- models
- routing
- specs
aliases:
- ROUTING-SIGNAL-CONTRACT
relations:
- type: refines
  target: GONI-IMAP-EB2133E6965D
- type: depends_on
  target: CONFIDENCE-01
- type: depends_on
  target: COG-GRAPH-01
sources: []
artifacts: []
uncertainty: "The signal vocabulary is an interface contract rather than a claim that every route needs every signal or that one universal routing function is optimal."
legacy: []
---

# Typed Cognitive Routing Signals

## 1. Purpose

The router should not become another semantic black box. Goni therefore
separates signal production from route selection.

For request or graph state x, a routing signal vector may be represented as:

z(x) = [
  task,
  domain,
  difficulty,
  confidence,
  privacy,
  consequence,
  freshness,
  context_need,
  latency_budget,
  monetary_budget,
  energy_budget,
  tool_need,
  network_eligibility,
  policy_eligibility
].

Each field retains its own semantics and provenance.

## 2. Signal classes

Deterministic signals SHOULD be computed directly when possible, including:

- context length and token budget;
- available hardware/model bundles;
- monetary and wall-clock budgets;
- current network state;
- tool availability;
- capability and policy eligibility;
- data classification already present in canonical state.

Probabilistic signals MAY include:

- semantic domain;
- task difficulty;
- likely relevance;
- predicted model sufficiency;
- semantic risk classification;
- expected verification difficulty.

Probability-bearing signals that affect consequential routing remain subject to
CAL-01, CONFIDENCE-01, SELECTIVE-01, and LOSS-01 as applicable.

## 3. Signal provenance

A RouteSignalSet SHOULD record:

- signal name and value;
- producer identity;
- deterministic or probabilistic class;
- source state or evidence refs;
- timestamp/version;
- calibration reference where applicable;
- validity or expiry where relevant.

## 4. Routing contract

The router maps:

Work Order + RouteSignalSet + eligible cognitive roles + policy projection

to:

selected cognitive mechanism or abstention/escalation.

The routing policy may be deterministic, learned, hybrid, or contextual-bandit
based, but its evidence and operating assumptions must remain inspectable.

## 5. Authority invariant

Routing signals can affect what cognitive mechanism runs next.

They cannot create capabilities, alter mandates, or bypass policy.

Policy eligibility is read from canonical or canonically derived authority
state; it is not inferred from model confidence.
