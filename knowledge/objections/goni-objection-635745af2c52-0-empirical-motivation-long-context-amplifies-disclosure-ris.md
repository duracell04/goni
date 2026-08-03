---
id: GONI-OBJECTION-635745AF2C52
title: '0. Empirical motivation: long context amplifies disclosure risk'
type: objection
status: draft
implementation_state: not_applicable
proposition: When untrusted text is retrieved (docs, web, email), it can carry instructions that alter behavior or tool use.
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/40-privacy-and-text-confinement.md
  heading: '0. Empirical motivation: long context amplifies disclosure risk'
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 0. Empirical motivation: long context amplifies disclosure risk

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 0. Empirical motivation: long context amplifies disclosure risk

When untrusted text is retrieved (docs, web, email), it can carry instructions
that alter behavior or tool use. Longer prompts widen this attack surface and
blur the data vs instruction boundary. This is a practical injection vector in
LLM-integrated systems. [[greshake2023-indirect-prompt-injection]]

Separately, long-context behavior is fragile and position-sensitive, which
makes prompt growth a weak reliability strategy and increases the odds that
unreviewed text slips into effectful paths. [[liu2023-lost-middle]]

This motivates the following invariants: the Text Confinement Theorem and the
restriction that Control/Execution planes never store raw text.
