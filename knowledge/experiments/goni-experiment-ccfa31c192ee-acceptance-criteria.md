---
id: GONI-EXPERIMENT-CCFA31C192EE
title: Acceptance criteria
type: experiment
status: draft
implementation_state: not_applicable
proposition: A correction-derived update names its scope, evidence, contradiction count, review status, and target seam.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/50-evidence/eval/EVID-HARNESS-02-correction-delta-compiler.md
  heading: Acceptance criteria
  revision: f91b4339e701c60c0d3508cf2bb3bd613ef2e36d
---

# Acceptance criteria

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Acceptance criteria

- A correction-derived update names its scope, evidence, contradiction count,
  review status, and target seam.
- A single correction cannot become a global stable default.
- Accepted updates link to memory diff refs and learning receipts.
- Learning receipts include source refs and summaries, not raw text by default.
- High-risk and constitutional updates require explicit approval.
- Regression tests fail if the system reintroduces the corrected behavior in
  matching scope.
- Replay shows improvement without safety, privacy, latency, or interruption
  regression.
