---
id: EVID-JEV-202611
title: 'External evidence: Jev typed System One judgments'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Jev exposes a hosted typed judgment interface whose evaluated single-pass classifications and
  rankings motivate testing decision-native models as replaceable Goni cognitive joints.
domains:
- agent
- software
- research
- market
aliases: []
relations:
- type: supports
  target: COG-JOINT-01
  note: Concrete interface precedent for bounded model judgments.
- type: supports
  target: COG-SPEC-01
  note: Provides a comparator for lower variable-cost judgment execution.
sources:
- SRC-TYPESAFE2026-JEV
- SRC-TYPESAFE2026-JUDGMENT
- SRC-TYPESAFE2026-JEV114
- SRC-TYPESAFE2026-JEV-PRICE
- SRC-JEVAI2026-DISCOVERY
- SRC-DEUSSER2026-JEV
- SRC-LI2026-JEV-COMPARISON
artifacts: []
uncertainty: The vendor architecture and training details are not independently established. External results
  concern particular Jev versions and datasets; hosted model price, terms, and availability may change.
legacy: []
---

# External evidence: Jev typed System One judgments

## Interface and source boundary

TypeSafe AI introduced Jev on 15 September 2026 as a System One model:
provide state and typed queries, receive bounded judgments rather than
autoregressively generated text. Its reported question families include
choice among predefined options, ordinal/rubric scoring, and Boolean
probability (Noul). Multiple questions concerning shared state can be
evaluated together. TypeSafe describes a parallel sampler and reinforcement
learning for calibrated decisions (RLCD); these are vendor descriptions,
not an independently established full technical specification. The 14 October
1.14 announcement adds experimental image, output and context features.

TypeSafe's 3 November clarification places judgment with the model and
decision/execution with application logic. This distinction is consonant
with Goni's existing probabilistic-cognition/deterministic-authority decision.

The discovery links were [jevai.net](https://jevai.net/) and
[Wikipedia: Jev (AI model)](https://en.wikipedia.org/wiki/Jev_(AI_model)).
Affiliation of the former with TypeSafe is unestablished; the latter could
not be retrieved during review. The vendor's own announcements and named
external preprints provide the evidence here. Jev is distinct from LeCun's JEPA.

## Empirical observations

Deußer et al. (arXiv:2609.37647, preprint) report a zero-shot study of
Jev 1.13.0 across 37 datasets: 346,009 requests for under USD 10,
strong classification scores on several datasets, and useful choice
calibration. The binary-probability threshold required adjustment on
some datasets; performance degraded on finer-grained, noisy and
low-resource tasks.

Li et al. (arXiv:2610.11978, preprint) report comparable knowledge and
commonsense multiple-choice performance versus general-purpose models
in evaluated settings, with weaker multi-step mathematics performance.
These results support testing specific joints rather than blanket replacement
of deliberative reasoning.

## Price and economics

As accessed 27 November 2026, the official TypeSafe help entry last updated
3 November lists USD 0.042 per million input tokens and free output tokens.
The supplied discovery website instead lists USD 0.084 per million input
tokens. Use current provider-specific official terms before procurement.
For illustration only, a million calls of exactly 1,000 billable input tokens
each at the direct listed rate have a USD 42 input-inference charge, before
other billing dimensions, infrastructure, retries, escalation, human review,
privacy/governance or error costs. This is an arithmetic estimate, not a Goni
benchmark or a pricing commitment.

Jev is a useful externally hosted comparator. Neither local runnable weights,
offline availability, vendor retention posture for a particular account,
nor measured Goni performance has been established. No product dependency
or data transfer is authorized by this evidence record.
