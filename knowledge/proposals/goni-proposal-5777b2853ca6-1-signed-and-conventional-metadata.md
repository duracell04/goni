---
id: GONI-PROPOSAL-5777B2853CA6
title: 1. Signed and conventional metadata
type: proposal
status: draft
implementation_state: specified_only
proposition: Meta says it reads industry-shared indicators, including information expressed through C2PA and IPTC standards.
domains:
- market
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/60-market/ai-media-provenance.md
  heading: 1. Signed and conventional metadata
  revision: fc0e3881f67979ba52b15bdcf2c3bc651981fbd8
---

# 1. Signed and conventional metadata

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 1. Signed and conventional metadata

Meta says it reads industry-shared indicators, including information expressed
through C2PA and IPTC standards. These indicators can identify the tool or
organization that signed a provenance record and can describe creation or edit
actions.

C2PA Content Credentials are cryptographically signed provenance assertions
bound to an asset. Signature validation can show that the assertions are
well-formed, associated with the asset, and have not been altered since
signing. It does **not** prove that the assertions exhaust the asset's history,
that the signer is infallible, or that the scene depicted is factually true.

Content Credentials are also not exclusive to AI. Cameras, news organizations,
and conventional editing software can use the same standard. The relevant
question is what the signed assertions say, not merely whether a C2PA manifest
exists.
