---
id: GONI-IMAP-FB50F302AC3E
title: 2.1 Endpoint
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'For the MVP, the normative endpoint is: ` ext POST /v1/chat/completions Host: 127.0.0.1:PORT (loopback by default) Content-Type: application/json ` The server **binds to loopback only** in default configuration.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 2.1 Endpoint
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 2.1 Endpoint

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.1 Endpoint

For the MVP, the normative endpoint is:

`	ext
POST /v1/chat/completions
Host: 127.0.0.1:PORT  (loopback by default)
Content-Type: application/json
`

The server **binds to loopback only** in default configuration.

> **Invariant API-1 (Local-only by default)**
> In default “local-trust” mode, the HTTP server must not listen on non-loopback interfaces without explicit configuration.
