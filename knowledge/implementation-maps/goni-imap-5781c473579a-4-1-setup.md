---
id: GONI-IMAP-5781C473579A
title: 4.1 Setup
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The model router faces a two-armed decision at each request: Action \(a_s\): answer with small model only.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: 4.1 Setup
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 4.1 Setup

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.1 Setup

The model router faces a two-armed decision at each request:

- Action \(a_s\): answer with small model only.  
- Action \(a_\ell\): escalate to large model.

We assume that:

- There is an underlying random variable \(Z\) (task difficulty) that determines which model is �good enough�.  
- A learned confidence predictor \(p(x)\) estimates the probability that the small model is sufficient given features of the request and preliminary small-model output.

This is a classic **contextual bandit** problem: features are the context, the two actions yield different rewards (quality vs cost), and the router�s goal is to minimise regret vs an oracle with knowledge of \(Z\).
