---
id: EVID-NEGATED-PROMPTS-01
title: 'Source claim: evaluated language models degrade on negated prompts'
type: evidence
status: draft
implementation_state: not_applicable
proposition: Jang, Ye, and Seo report substantial performance degradation on negated prompts across the evaluated model families, including inverse-scaling behavior in their experiments.
domains:
- research
- instructions
- models
aliases: []
relations:
- type: supports
  target: INSTR-REP-01
sources:
- SRC-JANG2022-NEGATED-PROMPTS
artifacts: []
uncertainty: The experiments concern specific tasks and older model families. They motivate Goni evaluation and representation preferences but do not establish a universal inability of modern language models to process negation.
legacy: []
---

# Source claim: evaluated language models degrade on negated prompts

Jang, Ye, and Seo compare original and negated prompts across multiple model families and report large performance gaps, including inverse-scaling effects in their evaluated settings.

For Goni, this is evidence for minimizing unnecessary dependence on negated trajectory instructions and measuring the effect on current local deployment profiles.

The source does not justify removing explicit prohibitions that define material security, legal, privacy, authority, or scope boundaries.
