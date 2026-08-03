---
id: GONI-IMAP-9DF05711E96C
title: 0.2.1 ITCR cascade (asymmetric, gated)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: Goni treats inference-time compute reasoning (ITCR) as a bounded, interrupt- driven service.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: 0.2.1 ITCR cascade (asymmetric, gated)
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 0.2.1 ITCR cascade (asymmetric, gated)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 0.2.1 ITCR cascade (asymmetric, gated)

Goni treats inference-time compute reasoning (ITCR) as a bounded, interrupt-
driven service. The default loop is low-power, and ITCR is activated only when
escalation predicates indicate that extra compute is worth the cost.

Stages:

1) Continuous state maintenance (encoders + predictor).
2) Cheap proposer (small model or heuristic plan).
3) Escalation policy (explicit predicates, hysteresis).
4) ITCR reasoner/verifier (bounded search + repair loop).
5) Commit under governance (policy validation + audit).

See `blueprint/30-specs/itcr.md` for budgets, triggers, and invariants.

---
