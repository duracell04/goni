---
id: EVID-GHA-STREAKS-2026
title: "Source claim: GitHub Actions failing streaks"
type: evidence
status: draft
implementation_state: not_applicable
proposition: "GitHub Actions failures frequently persist across runs and can start without a repository change."
domains:
- research
- delivery
aliases: []
relations:
- type: supports
  target: CI-ASSURANCE-01
sources:
- SRC-BARDSIRI2026-STUCK-ON-RED
artifacts: []
uncertainty: "October 2026 public preprint; associations from sampled workflows do not establish the benefit of a particular GONI CI topology."
legacy: []
---

# GitHub Actions failing streaks — source claim

**Observation.** The October 2026 preprint studies over 30.9 million workflow runs from over 43,000 repositories. About 75% of failing runs followed another failure; approximately one third of failing streaks began without a repository change. Multipurpose workflows showed more failing streaks at comparable size.

**GONI relevance.** Record external failure causes and repeated runs independently of source changes.

**Limit.** These results are not evidence that splitting or automatically rerunning a given workflow improves reliability.
