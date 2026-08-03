---
id: GONI-SYNTHESIS-23AEF6275CF7
title: 3. Sources remain sources
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'The runtime preserves an explicit derivation chain: | Stage | Meaning | Authority posture | | Original evidence | File, message, record, media object, observation, or other captured source | Preserved with identity, integrity, permission, and retention metadata.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/65-local-sovereign-knowledge-runtime.md
  heading: 3. Sources remain sources
  revision: 43414875152ae18f9977f21c9786b2d7025081ac
---

# 3. Sources remain sources

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Sources remain sources

The runtime preserves an explicit derivation chain:

| Stage | Meaning | Authority posture |
| --- | --- | --- |
| Original evidence | File, message, record, media object, observation, or other captured source | Preserved with identity, integrity, permission, and retention metadata. |
| Technical representation | OCR, transcript, parsed table, normalized record, or extracted structure | Fallible representation linked to the source and parser. |
| Machine enrichment | Embedding, entity link, classification, summary, inferred claim, or graph edge | Derived and untrusted until used under policy. |
| Human interpretation | A person's explanation, judgment, dissent, or contextual reading | Attributable interpretation, not a rewrite of the source. |
| Principal or delegate assertion | A scoped rule or position selected for operational use | Carries only the authority explicitly delegated to that actor and scope. |

A representation of a source MUST NOT silently replace the source. Derived
artifacts retain source refs, derivation stage, parser or model identity,
confidence, permissions, validity, and applicable receipts. If retention,
deletion, or redaction removes the original, derivatives must expose the
source as unavailable, redacted, or tombstoned rather than pretending to be
original evidence.

An owner assertion is not metaphysical truth. It is a scoped authority decision
for the owner's system. Model output cannot promote itself into owner-approved
fact, policy, memory, or operational authority.
