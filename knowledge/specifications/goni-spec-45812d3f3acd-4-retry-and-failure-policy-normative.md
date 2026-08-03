---
id: GONI-SPEC-45812D3F3ACD
title: 4. Retry and failure policy (normative)
type: specification
status: draft
implementation_state: specified_only
proposition: Every attempt MUST be receipted.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/metering/SPEC-METER-01-execution-metering.md
  heading: 4. Retry and failure policy (normative)
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 4. Retry and failure policy (normative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. Retry and failure policy (normative)
- Every attempt MUST be receipted.
- Retry policy MUST state whether resource attribution is per attempt or per successful execution.
- Automatic retries MUST keep parent execution linkage (`parent_execution_id`).
