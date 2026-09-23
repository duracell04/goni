---
id: EVID-SWEBENCH-EXECUTABLE-VERIFICATION-01
title: 'Source claim: SWE-bench uses executable repository outcomes'
type: evidence
status: draft
implementation_state: not_applicable
proposition: SWE-bench evaluates code-changing language-model systems on real repository issues where candidate patches are judged against executable tests and repository behavior.
domains:
- research
- evaluation
- software
aliases: []
relations:
- type: supports
  target: TRAJECTORY-EVAL-01
sources:
- SRC-JIMENEZ2023-SWEBENCH
artifacts: []
uncertainty: Software tests provide unusually objective verification; other delegated domains may require different state-based or human-grounded evaluators.
legacy: []
---

# Source claim: SWE-bench uses executable repository outcomes

SWE-bench is a useful reference for Goni's principle that successful generation is not equivalent to successful execution.

Where deterministic evaluators exist, Goni Lab should prefer them over model self-judgment.
