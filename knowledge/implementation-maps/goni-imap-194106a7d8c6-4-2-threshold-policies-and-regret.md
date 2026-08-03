---
id: GONI-IMAP-194106A7D8C6
title: 4.2 Threshold policies and regret
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Under assumptions: The confidence predictor is \(\epsilon\)-calibrated, i.e.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/95-theory-appendix.md
  heading: 4.2 Threshold policies and regret
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 4.2 Threshold policies and regret

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.2 Threshold policies and regret

Under assumptions:

- The confidence predictor is \(\epsilon\)-calibrated, i.e. predicted probabilities match empirical frequencies up to error \(\epsilon\).  
- Reward gaps between good and bad actions are bounded.

One can show that simple **threshold policies** on \(p(x)\) (accept vs escalate) can achieve **bounded average regret**, with the bound depending on calibration error and reward gaps.

Goni does not attempt to derive tight theoretical bounds at this stage; rather, it:

- Formalises regret \(R_T\) and its normalisation \(R_T/T\) as key metrics.  
- Specifies a target average regret (e.g. = 0.07) as a **design constraint**.  
- Validates policies empirically on labelled datasets.

This makes �small-then-big� routing a **controlled approximation**, not a hand-wavy optimisation.

---
