---
id: GONI-IMAP-394F7632390E
title: 5.1 Errors
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Typical error cases: 400 Bad Request – malformed JSON or clearly invalid fields.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 5.1 Errors
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 5.1 Errors

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.1 Errors

Typical error cases:

* 400 Bad Request – malformed JSON or clearly invalid fields.
* 429 Too Many Requests – explicit rate limit / overload response.
* 503 Service Unavailable – resources temporarily exhausted.
* 500 Internal Server Error – unexpected failure.

> **Invariant API-4 (no silent overload)**
> Overload or rate limiting must result in explicit HTTP errors (429/503); the node may not silently queue unboundedly in ways that violate the Control Plane’s latency and stability assumptions.
