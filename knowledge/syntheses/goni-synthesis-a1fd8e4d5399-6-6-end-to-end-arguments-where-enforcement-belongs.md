---
id: GONI-SYNTHESIS-A1FD8E4D5399
title: 6.6 End-to-end arguments (where enforcement belongs)
type: synthesis
status: draft
implementation_state: specified_only
proposition: End-to-end arguments show some integrity properties cannot be guaranteed by best-effort intermediates; enforcement must live at boundaries where checks are complete.
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/adjacent-projects.md
  heading: 6.6 End-to-end arguments (where enforcement belongs)
  revision: 774bf0138369e359249a7d02259c64a37a309ea7
---

# 6.6 End-to-end arguments (where enforcement belongs)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6.6 End-to-end arguments (where enforcement belongs)

End-to-end arguments show some integrity properties cannot be guaranteed by
best-effort intermediates; enforcement must live at boundaries where checks
are complete. [R9]

Goni boundary placement (normative):
- Tool capability syscalls enforce no ambient side effects.
- net.egress enforces no unlogged outbound traffic.
- Council escalation is explicit and logged.

The "Rise of the Middle" RFC supports keeping enforcement at explicit
boundaries rather than implicit middleboxes. [R10]
