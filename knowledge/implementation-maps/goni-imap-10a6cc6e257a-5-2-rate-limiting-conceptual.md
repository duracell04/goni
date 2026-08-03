---
id: GONI-IMAP-10A6CC6E257A
title: 5.2 Rate limiting (conceptual)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'The MVP may not enforce strong limits, but the *model* is: Per-identity counters: equests_per_minute, okens_per_minute, concurrent_requests.'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/40-apis-and-ui/api-surface.md
  heading: 5.2 Rate limiting (conceptual)
  revision: 0ddfb643686cb5c9e01b6bd8d5bee21add43ac7a
---

# 5.2 Rate limiting (conceptual)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 5.2 Rate limiting (conceptual)

The MVP may not enforce strong limits, but the *model* is:

* Per-identity counters:

  * 
equests_per_minute,
  * 	okens_per_minute,
  * concurrent_requests.
* Limits chosen so they are compatible with stability conditions in \(\mathcal{K}\) (utilisation < \(\alpha\)).

---
