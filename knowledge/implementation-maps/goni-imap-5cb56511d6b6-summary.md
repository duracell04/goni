---
id: GONI-IMAP-5CB56511D6B6
title: Summary
type: implementation-map
status: draft
implementation_state: specified_only
proposition: '**Encoders** map inputs to latent representations (vectors + minimal structured features).'
domains:
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/30-components/latent-predictor.md
  heading: Summary
  revision: a04290dad0b4572059e9ae4b0864fbaf1dbdd939
---

# Summary

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Summary

- **Encoders** map inputs to latent representations (vectors + minimal structured features).
- A **Predictor** updates latent state and selects actions (tools, memory, retrieval, planning).
- A **Decoder** is optional and used for language output and human-facing artifacts.

The system optimizes for correct latent state updates and correct tool actions, not for constant token emission.
