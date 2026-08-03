---
id: GONI-THESIS-5B195E38B2D8
title: '6. Authority: The Governance Layer'
type: thesis
status: draft
implementation_state: specified_only
proposition: The central primitive in Goni is authority.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/05-sovereign-delegation-os-thesis.md
  heading: '6. Authority: The Governance Layer'
  revision: c48031be8a4ca8ba57323dccb803c8dcdaab65dc
---

# 6. Authority: The Governance Layer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. Authority: The Governance Layer

The central primitive in Goni is authority. Authority answers the question:

```text
What may Goni do?
```

Goni's proposed answer is policy, mandates, autonomy corridors, and capability
tokens. In the target user experience, the principal does not approve every low-level action
manually. Instead, the principal defines higher-level mandates and bounded
corridors of autonomy. The system then proposes or acts within those corridors
and escalates when ambiguity, risk, or policy boundaries require review.

This model avoids two bad extremes. On one side is unsafe autonomy, where the
system silently acts beyond the principal's intent. On the other side is
unusable confirmation fatigue, where the assistant asks for approval so often
that delegation has no practical value. Goni's middle path is policy-level
governance:

```text
Set policy -> allow bounded execution -> review anomalies
```

The human-facing version of this governance model is simple. Users first
encounter product concepts rather than policy hashes, kernel mediation, or
capability token internals:

- Mandates define what the principal wants Goni to handle.
- Corridors define how much authority Goni has.
- Receipts explain what happened and why.
- Revocation allows the principal to withdraw authority.

Thus, the product translation of the kernel is:

```text
Mandates define goals.
Corridors define authority.
Receipts create trust.
Revocation preserves control.
```

Financial delegation makes this especially concrete. The
[Delegated Agent Treasury](/blueprint/30-specs/delegated-agent-treasury.md)
spec is a specified-only contract for bounded financial authority. It treats
commercial agents as delegated economic actors operating inside mandates, spend
caps, approval thresholds, evidence criteria, and revocation paths.
