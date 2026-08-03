---
id: GONI-IMAP-D9B234366562
title: 4. MVP position
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'For the MVP / prototype: There is **no mesh implementation**.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/mesh-and-wireguard.md
  heading: 4. MVP position
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# 4. MVP position

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4. MVP position

For the MVP / prototype:

- There is **no mesh implementation**.
- All traffic and scheduling remain strictly **local**.
- ??, ??, ??, ?? must *not* assume mesh exists; they only see a single-node environment.

This file exists to document the intended multi-node story and to constrain future design, not to mandate any implementation in v1.
