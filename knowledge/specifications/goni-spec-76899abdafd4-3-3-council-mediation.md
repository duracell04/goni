---
id: GONI-SPEC-76899ABDAFD4
title: 3.3 Council mediation
type: specification
status: draft
implementation_state: specified_only
proposition: Remote LLM traffic is just another egress class.
domains:
- network
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/network-gate-and-anonymity.md
  heading: 3.3 Council mediation
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 3.3 Council mediation

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 3.3 Council mediation

Remote LLM traffic is just another egress class. The Gate routes Council calls
either DIRECT or OVERLAY according to policy (see blueprint/docs/remote-llm-architecture.md).
Remote extraction of observed screen, app, browser, OCR, accessibility, or
audio context is egress. The Gate must mediate it even when the upstream
capture or extraction step was locally authorized.
