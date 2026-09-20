---
id: GONI-EVIDENCE-6F21B6FCEF3C
title: 'Source claim: zhang2023-h2o'
type: evidence
status: draft
implementation_state: not_applicable
proposition: H2O reports that attention value during generative inference is concentrated on a comparatively small subset of heavy-hitter tokens and uses that observation to motivate utility-aware KV-cache retention.
domains:
- research
- kernel
aliases: []
relations:
- type: supports
  target: GONI-SYNTHESIS-227700D474C6
sources:
- SRC-ZHANG2023-H2O
artifacts: []
uncertainty: The reported concentration is model-, workload-, layer-, and implementation-dependent. It does not establish that lexical Zipf frequency is a sufficient KV-importance estimator or that Goni memory access follows a universal power law.
legacy: []
---

# Source claim: zhang2023-h2o

H2O reports a heavy-hitter effect in generative transformer attention: a relatively
small subset of tokens accumulates a disproportionate share of attention value.
The paper proposes retaining those heavy hitters together with recent tokens when
the KV cache must be constrained.

For Goni, the relevant observation is narrower than lexical Zipf's law. It
supports testing whether future inference utility is sufficiently concentrated to
justify non-uniform cache allocation. It does not justify treating frequent token
identity as reusable KV state, because KV representations remain
context-dependent.

The source therefore supports a research direction: measure concentration at the
actual abstraction being optimized, then compare utility-aware retention against
uniform, recency-only, and fixed-budget baselines.
