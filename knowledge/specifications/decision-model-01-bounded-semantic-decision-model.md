---
id: DECISION-MODEL-01
title: Bounded Semantic Decision Model Contract
type: specification
status: draft
implementation_state: specified_only
proposition: "A bounded semantic decision model maps supplied state and typed, predefined question schemas to decisions or predictive distributions over bounded answer spaces without requiring natural-language generation as the output contract."
domains:
- models
- specs
- system
aliases:
- BOUNDED-DECISION-MODEL
relations:
- type: refines
  target: GONI-PRINCIPLE-HET-INTEL-01
- type: refines
  target: GONI-THESIS-AF4B6A0B4A2D
- type: depends_on
  target: GONI-DECISION-9AF49466170D
sources: []
artifacts: []
uncertainty: "This is a provider- and architecture-independent interface proposal. It does not assert that any particular model family is calibrated, faster, cheaper, or semantically superior."
legacy: []
---

# Bounded Semantic Decision Model Contract

## 1. Purpose

Many software decisions require semantic interpretation while the useful output
space is known before inference. Examples include boolean judgments,
classification among declared alternatives, ordinal rubric decisions, or scores
within an explicit interval.

For these tasks Goni should support a bounded decision primitive independently
of open-ended language generation.

## 2. Abstract interface

Let (s) denote supplied state and let

[
Q = \{q_1, \ldots, q_n\}
]

be typed question schemas. Each question (q_j) declares an answer space
(Y_j).

A decision model exposes the conceptual mapping

[
D(s,Q)
\rightarrow
\{\widehat{P}(Y_1\mid s,q_1),\ldots,
  \widehat{P}(Y_n\mid s,q_n)\}.
]

An implementation MAY return a selected answer together with scores or
probabilities instead of materializing a complete distribution when the
contract makes those semantics explicit.

Supported logical shapes may include:

- Boolean: (Y=\{false,true\});
- Choice<K>: one result from an explicitly declared finite set;
- Ordinal<K>: an ordered finite scale;
- BoundedScore[a,b]: a score whose legal range is fixed by the schema;
- Abstain: an explicit non-decision where the model or selector declines
  coverage.

## 3. Structural properties

The question schema MUST define the legal output domain before the result is
consumed by software.

Structural validity and semantic correctness are separate properties. A value
inside (Y_j) can still be the wrong semantic decision.

A probability-like field MUST NOT be described as calibrated merely because it
lies in ([0,1]). Calibration is governed separately.

## 4. Batched decision evaluation

The interface SHOULD permit several bounded questions to be evaluated against
the same supplied state in one request. This is an interface property; it does
not imply that a particular implementation evaluates those questions in
physical parallelism.

This allows a runtime to avoid repeated natural-language round trips when
multiple bounded judgments concern the same observation.

## 5. Relationship to generative models

A bounded decision result is useful when software needs a typed judgment.
Natural-language generation remains appropriate when the required artifact is
itself linguistic, such as an explanation, draft, synthesis, or conversation.

A system MAY use the same underlying model family for both interfaces, but the
contracts remain distinct.

## 6. Authority invariant

A decision-model result is cognitive evidence.

It is never, by itself:

- a capability token;
- a policy grant;
- a mandate;
- a revocation decision;
- permission to create an external effect.

Before consequential execution, the kernel evaluates canonical authority state
under the applicable policy contract.

Formally,

[
\text{DecisionModelOutput} \not\Rightarrow \text{Authority}.
]

## 7. Provider independence

The contract does not privilege a vendor, model architecture, training method,
local runtime, or hosted service. Providers are replaceable implementations
whose empirical properties must be established by evidence and matched
evaluation.
