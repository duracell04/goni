---
id: GONI-SPEC-E44D18525669
title: 7.2 Anonymous Mode (opt-in)
type: specification
status: draft
implementation_state: specified_only
proposition: 'Route default: OVERLAY for selected purposes or all external flows.'
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
  heading: 7.2 Anonymous Mode (opt-in)
  revision: c934b1fd5e3eaf4aaf3d931565c9665c24b62f8b
---

# 7.2 Anonymous Mode (opt-in)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 7.2 Anonymous Mode (opt-in)

- Route default: OVERLAY for selected purposes or all external flows.
- Direct egress denied unless explicitly allowed.
- Strict logging discipline; receipts avoid destinations/URLs by default.
- Web/search seats, if used, run through the Overlay Capsule.

These are policy bundles, not bypass switches; the Gate remains the only egress
chokepoint in all modes.
