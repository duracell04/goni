---
id: GONI-SYNTHESIS-B1D94E9EF74C
title: 5. Promotion gates (how fixes become durable)
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Fixes follow an explicit pipeline: Inbox: raw failure packet, untrusted.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/50-learning-loop.md
  heading: 5. Promotion gates (how fixes become durable)
  revision: facf4ec813a02ec315fbe482a25bdac18686846e
---

# 5. Promotion gates (how fixes become durable)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Promotion gates (how fixes become durable)
Fixes follow an explicit pipeline:

1. Inbox: raw failure packet, untrusted.
2. RFC: structured proposal with target seam, promotion class, and rollback
   pointer.
3. SPEC: formal contract or schema change, if needed.
4. ADR: decision record with rationale and alternatives.
5. EVID: evaluation results, non-regression report, and replay artefacts.

Minimum promotion evidence by class:

- P0: scoped task improvement plus reproducible replay on the affected suite.
- P1: P0 evidence plus safety and latency non-regression.
- P2: P1 evidence plus explicit approval, durable gains across repeated runs,
  and signed bundle metadata for rollback.

Adapter and worldview-lens promotion additionally requires comparative
evaluation against at least one neutral or critic lens. The promotion record
must show the expected behavior change, task scope, eval refs, active model
stack refs, approval state, and rollback path.

Delegation-policy promotion specifically requires trace replay over vague-intent
episodes, comparison against prior policy bundles, and review of question rate,
override rate, unsafe autonomy, and surfaced-assumption coverage. Failed
bundles must be rollbackable through the governance ledger.

Harness-policy promotion additionally requires a falsifiable change statement:
target component, expected outcome delta, evidence sources, eval window,
retention rule, and rollback condition. A change that improves a narrow task but
increases interruption rate, policy violations, unreviewed egress, or rollback
frequency fails promotion unless an explicit higher-level policy accepts that
trade-off.
