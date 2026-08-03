---
id: GONI-IMAP-6E9EEDC3BA3B
title: 5. MVP vs future runtime
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**MVP** 1–2 local models (goni-small, goni-large).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/llm-runtime.md
  heading: 5. MVP vs future runtime
  revision: 6ce37ef5d3a676fd26377a3fa8a15c5b226016c2
---

# 5. MVP vs future runtime

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. MVP vs future runtime

**MVP**

* 1–2 local models (goni-small, goni-large).
* Single device type per session.
* No cross-session KV reuse beyond what backend provides.

**Future**

* Multi-device and multi-backend routing inside ??.
* Advanced KV cache paging tightly integrated with ??.
* Mixed local/cloud execution under the same interface.
* Signed bundle catalogs and per-bundle attestation receipts.
