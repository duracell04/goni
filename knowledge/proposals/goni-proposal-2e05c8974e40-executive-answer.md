---
id: GONI-PROPOSAL-2E05C8974E40
title: Executive answer
type: proposal
status: draft
implementation_state: specified_only
proposition: Meta does not need to infer an "AI style" from appearance alone.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/ai-media-provenance.md
  heading: Executive answer
  revision: fc0e3881f67979ba52b15bdcf2c3bc651981fbd8
---

# Executive answer

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Executive answer

Meta does not need to infer an "AI style" from appearance alone. Its published
approach combines provenance signals and platform context:

1. industry-standard metadata, including C2PA and IPTC signals;
2. invisible watermarks or related pixel-level markers;
3. direct knowledge that Meta's own generative tools were used;
4. disclosures supplied by the person or advertiser posting the media; and
5. classifiers intended to detect some generated media when markers are absent.

No one signal is complete. Metadata can be stripped, watermarks can be
degraded, classifiers can fail, and a valid provenance record does not prove
that the depicted event is true. An "AI info" label therefore means that Meta
received or detected evidence of AI involvement; it does not by itself mean
that the whole asset is fabricated.
