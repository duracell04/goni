---
id: COG-GRAPH-01
title: Governed Cognitive Graph
type: specification
status: draft
implementation_state: specified_only
proposition: "Goni should represent multi-step cognition as a typed, budgeted graph of replaceable cognitive mechanisms whose transitions are explicit and auditable, while authority for external effects remains exclusively kernel-owned."
domains:
- agent
- models
- specs
- system
aliases:
- COGNITIVE-GRAPH
relations:
- type: refines
  target: GONI-PRINCIPLE-HET-INTEL-01
- type: depends_on
  target: GONI-PRINCIPLE-GOV-LOOP-01
- type: depends_on
  target: DECISION-MODEL-01
- type: refines
  target: GONI-SPEC-FD6CC59ECC08
sources: []
artifacts: []
uncertainty: "This specification defines a model- and framework-independent graph contract. Concrete schedulers, graph runtimes, optimization algorithms, and node implementations require separate evidence and implementation work."
legacy: []
---

# Governed Cognitive Graph

## 1. Purpose

Goni already supports heterogeneous cognition, escalation, verification, and
kernel-mediated action. The cognitive graph makes the composition of those
mechanisms explicit.

A cognitive graph is a directed graph:

G = (V, E)

where each node v in V is one bounded cognitive operation and each edge e in E
declares an allowed transition condition.

The graph is not an authority graph. It decides what computation may be
attempted next. The kernel still decides whether an external effect is
permitted.

## 2. Cognitive node contract

A cognitive node SHOULD declare at least:

- node_id;
- role_ref or operation role;
- input_schema_ref;
- output_schema_ref;
- mechanism_selector_ref;
- context or state references permitted to enter the node;
- compute, latency, token, energy, or monetary budget where applicable;
- privacy and egress class;
- acceptance or abstention rule where probabilistic output is used;
- verification requirement;
- retry or iteration budget;
- receipt/provenance requirements for consequential routing decisions.

A node may be implemented by deterministic code, retrieval, a classifier, a
bounded decision model, a generative model, a reasoning model, a verifier,
another governed agent, or another approved mechanism.

## 3. Typed transitions

Edges SHOULD use finite, explicit transition semantics such as:

- on_success;
- on_abstain;
- on_low_confidence;
- on_verification_failure;
- on_budget_exhaustion;
- on_policy_denial;
- on_timeout;
- on_external_state_change.

Transition predicates may use deterministic state or probabilistic evidence,
but probabilistic evidence MUST remain subject to the applicable calibration
and consequence-control contracts.

## 4. Composition patterns

The graph MAY express:

### Sequence

v1 -> v2 -> v3.

### Fan-out

One node may dispatch independent bounded work to multiple specialist nodes
subject to graph and scheduler budgets.

### Fan-in

A verifier or synthesizer may consume outputs from several predecessor nodes
when the input provenance is preserved.

### Bounded repair loop

generate -> verify -> repair -> verify

is permitted only with an explicit iteration budget and stop condition.

### Conditional escalation

A cheap mechanism may route to a more expensive mechanism when uncertainty,
verification failure, consequence, freshness, capability, or expected value
justifies escalation.

## 5. Graph invariants

The following invariants apply:

1. Every node and edge has a stable identifier or reproducible definition.
2. Every graph execution is bound to a Work Order and its budgets.
3. Cognitive graph transitions cannot enlarge the Work Order mandate.
4. A downstream node receives only the context, tools, and network eligibility
   permitted for that node.
5. A graph cycle requires a finite iteration or resource bound.
6. External effects still require kernel policy, capability validation, and the
   normal receipt path.
7. Graph routing evidence cannot become canonical authority state.

Formally:

delta cognitive route does not imply delta authority.

## 6. Relationship to ITCR

ITCR remains a valid specialized cognitive graph:

low-power state
-> cheap proposal
-> escalation control
-> reasoner/verifier
-> governed commit.

COG-GRAPH-01 generalizes that fixed cascade into a reusable composition
contract without replacing the ITCR-specific semantics.

## 7. Receipts and replay

A graph execution SHOULD permit reconstruction of:

- graph/version selected;
- nodes attempted;
- mechanisms considered and selected;
- transition predicates evaluated;
- confidence/calibration evidence where relevant;
- verification results;
- budgets consumed;
- policy denials or escalations;
- final output and effect receipts.

The graph is therefore an inspectable cognitive trajectory rather than hidden
prompt glue.
