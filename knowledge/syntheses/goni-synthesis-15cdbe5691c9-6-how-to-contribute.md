---
id: GONI-SYNTHESIS-15CDBE5691C9
title: 6. How to contribute
type: synthesis
status: draft
implementation_state: specified_only
proposition: '**Read the constraints (Section 2)** - treat them as the current baseline.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/README.md
  heading: 6. How to contribute
  revision: 7f25b6c35b7c08fa87b3fdc9624fd60c5b81bffb
---

# 6. How to contribute

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 6. How to contribute

1. **Read the constraints (Section 2)** - treat them as the current baseline.  
2. For **hardware topics**, open:
   - an issue with label `hardware`, and  
   - reference the relevant file in `/hardware/`.  

3. For **software topics**, label `software` and reference `/software/`.  

4. Major changes should come with:
   - a short proposal (`proposal-*.md`) and  
   - a suggested update to [blueprint/hardware/90-decisions.md](/blueprint/hardware/90-decisions.md) or [blueprint/software/90-decisions.md](/blueprint/software/90-decisions.md).

5. Keep discussion as concrete as possible:  
   - numbers (W, GB, TB, TOPS, latency)  
   - resource and performance estimates  
   - actual vendor links if relevant

We are optimising for a clean, well-argued architecture. Changes that affect constraints/architecture should come with numbers and a short proposal.

If you are thinking about manufacturing/funding, open an issue; the hardware build triggers when the blueprint feels solid and a partner is ready.

---
