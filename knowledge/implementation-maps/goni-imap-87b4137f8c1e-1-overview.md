---
id: GONI-IMAP-87B4137F8C1E
title: 1. Overview
type: implementation-map
status: draft
implementation_state: specified_only
proposition: The **API surface** is the formal contract between a Goni node and external clients.
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 1. Overview
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 1. Overview

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1. Overview

The **API surface** is the formal contract between a Goni node and external clients.

We model the node as computing (possibly with randomness):

\mathsf{Serve} : \mathsf{Req}_{\text{Goni}} \to \mathsf{Stream}(\text{Token}) \times \mathsf{Log}


where:

- \(\mathsf{Req}_{\text{Goni}}\) = set of valid HTTP requests under /v1/chat/completions,
- Stream(Token) = a (possibly streaming) sequence of output tokens,
- Log = internal metrics, traces, and tool results written into \(\mathcal{A}\).

All other components (CLI, dashboard, plugins) are clients of this function.

---
