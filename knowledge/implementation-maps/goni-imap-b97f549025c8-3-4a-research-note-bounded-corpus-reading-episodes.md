---
id: GONI-IMAP-B97F549025C8
title: '3.4a Research note: bounded corpus-reading episodes'
type: implementation-map
status: draft
implementation_state: specified_only
proposition: If the long-context research lane is exercised, the Control Plane may schedule bounded corpus-reading episodes in the same spirit as other tool-mediated reasoning bursts.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/20-architecture.md
  heading: '3.4a Research note: bounded corpus-reading episodes'
  revision: 2614ed8e6086127429c089440726103798a0a9bf
---

# 3.4a Research note: bounded corpus-reading episodes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.4a Research note: bounded corpus-reading episodes

If the long-context research lane is exercised, the Control Plane may schedule
bounded corpus-reading episodes in the same spirit as other tool-mediated
reasoning bursts.

- These episodes are experimental and do not change the default context path.
- Recursive or parallel subreads must remain subject to explicit budget limits.
- If the lane ever becomes normative, the scheduler and receipt contracts will
  need dedicated fields for scan/subread accounting.
