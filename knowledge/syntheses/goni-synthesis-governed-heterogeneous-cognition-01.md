---
id: GONI-SYNTHESIS-GOVERNED-HETEROGENEOUS-COGNITION-01
title: Governed Heterogeneous Cognition
type: synthesis
status: draft
implementation_state: not_applicable
proposition: "Goni composes heterogeneous cognitive mechanisms through a governed cognitive graph whose nodes request functional roles, whose router consumes typed evidence-bearing signals, whose agent identity remains independent from model substrate, and whose effects remain subject to canonical kernel policy and receipts."
domains:
- agent
- models
- routing
- system
aliases:
- GOVERNED-COGNITIVE-GRAPH-SYNTHESIS
relations:
- type: synthesizes
  target: GONI-SYNTHESIS-PROBABILISTIC-DECISION-CONTROL-01
- type: synthesizes
  target: COG-GRAPH-01
- type: synthesizes
  target: AGENT-MODEL-ORTHO-01
- type: synthesizes
  target: MODEL-ROLE-01
- type: synthesizes
  target: ROUTE-SIGNAL-01
- type: synthesizes
  target: POLICY-PROJECTION-01
- type: synthesizes
  target: CONTEXT-ALLOC-01
- type: synthesizes
  target: GONI-EXPERIMENT-COG-GRAPH-01
sources: []
artifacts: []
uncertainty: "This synthesis combines specified contracts and evidence. It does not claim that dynamic heterogeneous graphs outperform simpler architectures until the matched experiment is run."
legacy: []
---

# Governed Heterogeneous Cognition

PR #19 established the probabilistic decision-control path:

heterogeneous inference
-> calibrated uncertainty
-> selective accept or abstain
-> consequence-sensitive control
-> canonical authorization
-> mediated action
-> verification and receipts
-> recalibration.

The cognitive-graph layer answers the next question:

How should multiple different cognitive mechanisms be composed before an effect
reaches that authority boundary?

## 1. Work Order to cognitive graph

A Work Order binds objective, constraints, risk, budgets, tools, authority, and
Done Contract.

The orchestrator selects or instantiates a governed cognitive graph whose nodes
perform bounded cognitive operations.

Each node asks for a functional role rather than a provider name.

Examples include:

- deterministic computation;
- retrieval;
- classifier;
- bounded decision model;
- proposer;
- generator;
- reasoner;
- critic;
- verifier;
- guard;
- compressor;
- synthesizer;
- perception.

## 2. Agent identity remains stable

The agent is the accountable process.

Models are replaceable cognitive substrates.

A coding agent can use a local classifier, a bounded decision model, a small
local generator, a remote frontier reasoner, and a verifier during one
trajectory without changing agent identity or authority.

This preserves continuity of memory, mandate, tools, policy, and receipts while
allowing cognitive specialization.

## 3. Signals select mechanisms

Routing consumes typed signals produced by deterministic and probabilistic
mechanisms.

Examples include:

task/domain
+ difficulty
+ calibrated confidence
+ privacy
+ consequence
+ freshness
+ context need
+ latency/cost/energy budget
+ tool need
+ policy eligibility.

The router resolves those signals against eligible cognitive roles and approved
model/mechanism bundles.

## 4. Graph composition

The graph can express:

- sequences;
- conditional escalation;
- fan-out to independent or specialist nodes;
- fan-in to verification or synthesis;
- bounded generate-verify-repair loops.

Complexity is optional. The smallest graph that satisfies the Done Contract and
assurance requirements should be preferred.

## 5. Policy remains singular

Canonical kernel policy remains the authority source.

Surface-specific eligibility for model routes, graph nodes, tools, network
egress, MCP, or agent-to-agent communication may be compiled as derived
projections bound to the canonical policy hash.

A projection may narrow authority. It cannot expand it.

A graph edge determines what cognition may run next. It does not grant an
external effect.

## 6. Context is allocated at three levels

Cognitive resource allocation distinguishes:

L1: source/evidence selection;
L2: semantic text compression;
L3: model-internal KV residency.

Each layer discards or compresses different information and therefore requires
different evidence, recovery mechanisms, and evaluation.

## 7. Complete governed path

The resulting architecture is:

Work Order
-> policy projection and route signals
-> governed cognitive graph
-> role-based mechanism selection
-> cognition / verification
-> calibrated decision control
-> kernel authorization
-> mediated tool action
-> observe and verify result
-> receipts and canonical state
-> routing/calibration evidence update.

In compact form:

Models and algorithms provide cognition.
The cognitive graph composes it.
Typed signals route it.
Calibration qualifies probabilistic evidence.
Statistical control accepts or escalates it.
Canonical policy constrains it.
The kernel authorizes effects.
Tools act.
Verification observes.
Receipts prove.
Outcomes improve future routing without rewriting authority.

## 8. Complexity discipline

The graph is not an argument for maximal orchestration.

Every additional node, model, agent, provider, loop, and policy projection adds
latency, cost, failure modes, and verification burden.

GONI-EXPERIMENT-COG-GRAPH-01 therefore compares the graph against simpler
alternatives and permits the conclusion that one strong model or a fixed
cascade is preferable for a particular workload.
