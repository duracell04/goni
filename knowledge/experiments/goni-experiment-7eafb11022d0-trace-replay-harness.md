---
id: GONI-EXPERIMENT-7EAFB11022D0
title: Trace replay harness
type: experiment
status: draft
implementation_state: not_applicable
proposition: Delegation evaluation uses trace replay rather than ad-hoc screenshots or anecdotal demos.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/README.md
  heading: Trace replay harness
  revision: 1fce8b46efaf612f63f99ec32300ebf4da32522f
---

# Trace replay harness

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Trace replay harness

Delegation evaluation uses trace replay rather than ad-hoc screenshots or
anecdotal demos.

Replay suites should include:

- vague intent prompts with gold deliverables,
- policy bundles that vary corridor, clarification, and visibility rules,
- traces that distinguish delegation from co-creation,
- receipts and audit traces for each mediated action,
- outcome labels covering success, overrides, unsafe autonomy, and question
  usefulness.

Minimum reported dimensions:

- quality/success,
- turns and latency,
- user corrections and overrides,
- interaction-mode classification accuracy,
- surfaced-assumption coverage,
- question count and question value,
- branch_count / variant_count_requested under fixed budgets.

Long-context reading evaluation should additionally report:

- reading strategy used,
- span/citation fidelity,
- scan/slice/subread counts,
- recursion depth,
- whether the strategy underperformed the native in-window baseline.
