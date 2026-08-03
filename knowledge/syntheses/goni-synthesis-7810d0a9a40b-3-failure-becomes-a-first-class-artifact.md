---
id: GONI-SYNTHESIS-7810D0A9A40B
title: 3. Failure becomes a first-class artifact
type: synthesis
status: draft
implementation_state: specified_only
proposition: Every failure produces an experience packet derived from receipts and runtime state.
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/50-learning-loop.md
  heading: 3. Failure becomes a first-class artifact
  revision: facf4ec813a02ec315fbe482a25bdac18686846e
---

# 3. Failure becomes a first-class artifact

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Failure becomes a first-class artifact
Every failure produces an experience packet derived from receipts and runtime
state. This enables repeatable repairs without claiming the model "learns."

Minimum fields:
- failure_id, timestamp, request_id
- receipt_ids (immutable chain)
- failure_class (see Section 4)
- observed_symptoms (short tags)
- proposed_fix (promotion class + target seam)
- evidence_links (metrics, logs, or traces)
