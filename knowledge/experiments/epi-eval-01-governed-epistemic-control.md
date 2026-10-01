---
id: EPI-EVAL-01
title: Governed Epistemic Control Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Evaluate whether explicit transient epistemic-state tracking with competing hypotheses, lineage-aware evidence, discriminating retrieval, and explicit stopping criteria reduces unsupported or premature commitments relative to ordinary model-directed retrieval under matched model and resource budgets.
domains:
- agent
- research
- software
- system
aliases:
- EPISTEMIC-CONTROL-EVAL
relations:
- type: tests
  target: EPI-CTRL-01
sources:
- SRC-LINDLEY1956-EXPERIMENT-INFORMATION
- SRC-SETTLES2009-ACTIVE-LEARNING
- SRC-JIANG2023-FLARE
- SRC-ASAI2024-SELF-RAG
- SRC-GAO2023-RARR
artifacts: []
uncertainty: No net benefit is assumed. Explicit epistemic control may add latency, token cost, unnecessary branching, brittle heuristics, false source-independence judgments, or over-abstention. The evaluation must report neutral and negative results.
legacy: []
---

# Governed Epistemic Control Evaluation

## Hypothesis

Under matched model, retrieval, token, latency, tool, and connector budgets,
explicit epistemic-state tracking with competing hypotheses, lineage-aware
evidence, discriminating retrieval, and explicit stopping criteria reduces
unsupported or premature commitments relative to ordinary model-directed
retrieval without imposing unacceptable total-system cost.

This is a falsifiable research hypothesis, not an architectural guarantee.

## Baselines

Compare at minimum:

1. **Single-pass grounded generation:** compile the initial context, permit the
   baseline retrieval path, and answer without an explicit EpistemicFrame.
2. **Model-directed iterative retrieval:** permit the model to request further
   retrieval or context misses under the same overall budget, without explicit
   hypothesis, lineage, or discriminating-query state.
3. **Governed epistemic control:** EPI-CTRL-01 with explicit hypotheses,
   support/contradiction references, evidence gaps, bounded discriminating
   retrieval, and commit/continue/uncertainty termination.

Where practical, add ablations for:

- hypothesis tracking without lineage awareness;
- lineage awareness without discriminating query selection;
- epistemic state with heuristic query selection;
- calibrated information-value selection where a valid estimator exists.

## Evaluation fixtures

Construct synthetic, open, or replayable fixtures containing:

- an early plausible but incorrect hypothesis;
- two or more explanations consistent with the initial observations;
- a decisive item absent from the initial working set;
- many superficially independent sources that share one underlying source;
- genuinely independent corroborating sources;
- contradictory sources with different provenance or authority;
- stale evidence superseded by later evidence;
- evidence gaps that cannot be resolved within the permitted budget;
- cases where additional retrieval has negligible decision value;
- cases where no strong conclusion is justified;
- prompt or context patterns likely to induce confirmation-seeking queries.

Fixtures SHOULD label which evidence is decisive, which items share a lineage,
which hypotheses remain viable at each stage, and when abstention is the
appropriate outcome.

## Outcome metrics

Report:

- final conclusion accuracy where ground truth is available;
- unsupported-commit rate;
- premature-commit rate;
- correct uncertainty or abstention rate;
- contradiction-resolution rate;
- decisive-evidence recovery rate;
- alternative-hypothesis recall;
- wrong-leading-hypothesis rejection rate.

A higher number of hypotheses or retrieval calls is not itself a positive
outcome.

## Evidence-lineage metrics

Report:

- evidence-lineage independence errors;
- duplicate-source amplification;
- synthetic self-corroboration incidents;
- false independence assignments;
- missed independent corroboration;
- cases where raw source count materially diverges from effective independent
  support.

Hard target for any profile presented as audit-grade:

- model-generated descendants counted as independent confirmation: **0**.

## Query-quality metrics

Report:

- useful evidence recovered per retrieval;
- decisive evidence recovered per retrieval;
- irrelevant retrieval rate;
- repeated or redundant query rate;
- discriminating-query success rate;
- retrievals that change the support ordering among active hypotheses;
- information-value estimator calibration where a probabilistic estimator is
  evaluated.

For heuristic profiles, report the named heuristic rather than retroactively
interpreting heuristic scores as calibrated expected information gain.

## Stopping metrics

Report:

- correct commit rate;
- correct continue-search rate;
- correct uncertainty-surfacing rate;
- unnecessary continuation rate;
- premature stopping rate;
- budget-exhaustion-to-certainty incidents;
- research stopping regret where a counterfactual continuation can be scored.

Hard target:

- budget exhaustion silently converted into certainty: **0**.

## Cost and service metrics

Report:

- input and output tokens;
- retrieval and connector calls;
- solver/model calls;
- latency and wall-clock time;
- monetary cost where applicable;
- compute time and energy/thermal proxy where practical;
- peak memory or KV pressure where observable;
- user interruptions or clarification requests;
- receipt and state volume.

The epistemic-control condition must be evaluated on total-system cost rather
than answer quality alone.

## Adversarial hypothesis-locking test

Create cases in which an initially plausible hypothesis can generate highly
supportive but poorly discriminating searches.

Measure whether each condition:

1. generates or preserves a competing explanation;
2. issues a query capable of falsifying or distinguishing the leading
   hypothesis;
3. changes support when contrary evidence is recovered;
4. avoids treating repeated retrieval from the same lineage as independent
   confirmation;
5. reaches the appropriate commit or uncertainty state.

This lane directly tests whether retrieval volume becomes confirmation rather
than verification.

## Promotion criterion

EPI-CTRL-01 remains specified-only until a named implementation demonstrates,
on at least one GONI-relevant evidence-sensitive workload:

1. lower unsupported or premature commitment than the strongest matched
   baseline;
2. no synthetic self-corroboration counted as independent evidence in the
   audit-grade profile;
3. bounded branch, retrieval, and recursion behavior;
4. acceptable latency, compute, and acquisition cost;
5. calibrated or explicitly non-calibrated query-value semantics;
6. reconstructable commit, continue, and uncertainty decisions.

Negative or neutral results are valid outcomes and SHOULD narrow, simplify, or
reject the proposed mechanism rather than being hidden by architectural
narrative.
