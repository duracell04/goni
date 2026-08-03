---
id: GONI-EVIDENCE-316D2542E38A
title: 'Source claim: deepseek2026-deepspec'
type: evidence
status: draft
implementation_state: not_applicable
proposition: DeepSpec is DeepSeek's public, MIT-licensed codebase for training and evaluating speculative decoding algorithms, including DSpark-related draft model artifacts.
domains:
- research
aliases: []
relations:
- type: supports
  target: LLM-RUNTIME
sources:
- SRC-DEEPSEEK2026-DEEPSPEC
artifacts: []
uncertainty: The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.
legacy:
- path: blueprint/docs/references/bibliography.md
  heading: 'Key: [[deepseek2026-deepspec]]'
  revision: d6b7d35f8b13fd57bda38182abfaaaa6a1b048a6
---

# Source claim: deepseek2026-deepspec

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

Key: [[deepseek2026-deepspec]]
Claim: DeepSpec is DeepSeek's public, MIT-licensed codebase for training and
evaluating speculative decoding algorithms, including DSpark-related draft
model artifacts.
Relevance:
- Gives Goni a concrete reference for treating draft models as trainable and
  evaluable components behind the runtime interface.
Used in:
- `blueprint/software/30-components/llm-runtime.md`
Source:
- https://github.com/deepseek-ai/DeepSpec
